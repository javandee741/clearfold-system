use core::{mem, ptr};

use clearfold_boot_abi::{BootInfo, MemoryAttributes, MemoryKind, MemoryRegion};

use uefi::boot::{self, AllocateType, MemoryType};
use uefi::mem::memory_map::MemoryMap;

const PAGE_SIZE: usize = 4096;

/// Headroom for descriptor splitting caused by allocations and
/// Clearfold ownership reservations.
const MEMORY_MAP_SLACK: usize = 64;

pub struct BootPayload {
    info: *mut BootInfo,
    regions: *mut MemoryRegion,
    capacity: usize,

    allocation_base: u64,
    allocation_size: u64,
}

impl BootPayload {
    pub fn allocate() -> Self {
        let probe =
            boot::memory_map(MemoryType::LOADER_DATA).expect("cannot probe UEFI memory map");

        let descriptor_count = probe.entries().count();

        drop(probe);

        let capacity = descriptor_count
            .saturating_mul(2)
            .saturating_add(MEMORY_MAP_SLACK)
            .max(128);

        let regions_offset = align_up(mem::size_of::<BootInfo>(), mem::align_of::<MemoryRegion>());

        let bytes = regions_offset
            .checked_add(
                capacity
                    .checked_mul(mem::size_of::<MemoryRegion>())
                    .expect("Boot ABI region capacity overflow"),
            )
            .expect("Boot ABI payload size overflow");

        let pages = bytes.div_ceil(PAGE_SIZE);

        let allocation =
            boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages)
                .expect("cannot allocate Clearfold Boot ABI payload");

        let base = allocation.as_ptr();

        let allocation_size = pages
            .checked_mul(PAGE_SIZE)
            .expect("Boot ABI allocation size overflow");

        unsafe {
            ptr::write_bytes(base, 0, allocation_size);
        }

        let info = base.cast::<BootInfo>();

        let regions = unsafe { base.add(regions_offset) }.cast::<MemoryRegion>();

        Self {
            info,
            regions,
            capacity,

            allocation_base: base as u64,
            allocation_size: allocation_size as u64,
        }
    }

    /// Convert the final firmware map into Clearfold-owned memory
    /// semantics.
    ///
    /// No allocations and no UEFI Boot Services are allowed here.
    pub unsafe fn finalize<M: MemoryMap>(
        &mut self,
        memory_map: &M,
        kernel_phys_base: u64,
        kernel_phys_size: u64,
    ) -> Option<*const BootInfo> {
        let mut region_count =
            unsafe { normalize_memory_map(memory_map, self.regions, self.capacity) }?;

        unsafe {
            sort_regions(self.regions, region_count);
        }

        if !unsafe { validate_non_overlapping(self.regions, region_count) } {
            return None;
        }

        region_count = unsafe { merge_adjacent(self.regions, region_count) }?;

        let kernel_region = MemoryRegion::new(
            kernel_phys_base,
            kernel_phys_size,
            MemoryKind::KERNEL,
            kernel_attributes(),
        );

        region_count =
            unsafe { reserve_range(self.regions, region_count, self.capacity, kernel_region) }?;

        let boot_info_region = MemoryRegion::new(
            self.allocation_base,
            self.allocation_size,
            MemoryKind::BOOT_INFO,
            boot_info_attributes(),
        );

        region_count =
            unsafe { reserve_range(self.regions, region_count, self.capacity, boot_info_region) }?;

        region_count = unsafe { merge_adjacent(self.regions, region_count) }?;

        if !unsafe { validate_non_overlapping(self.regions, region_count) } {
            return None;
        }

        if !unsafe {
            range_is_kind(
                self.regions,
                region_count,
                kernel_phys_base,
                kernel_phys_size,
                MemoryKind::KERNEL,
            )
        } {
            return None;
        }

        if !unsafe {
            range_is_kind(
                self.regions,
                region_count,
                self.allocation_base,
                self.allocation_size,
                MemoryKind::BOOT_INFO,
            )
        } {
            return None;
        }

        if region_count > u32::MAX as usize {
            return None;
        }

        let mut info = BootInfo::empty();

        info.memory_regions_ptr = self.regions as u64;

        info.memory_region_count = region_count as u32;

        info.kernel_phys_base = kernel_phys_base;

        info.kernel_phys_size = kernel_phys_size;

        unsafe {
            ptr::write(self.info, info);
        }

        Some(self.info.cast_const())
    }
}

const fn align_up(value: usize, alignment: usize) -> usize {
    debug_assert!(alignment.is_power_of_two());

    (value + alignment - 1) & !(alignment - 1)
}

unsafe fn normalize_memory_map<M: MemoryMap>(
    map: &M,
    output: *mut MemoryRegion,
    capacity: usize,
) -> Option<usize> {
    let mut count = 0usize;

    for descriptor in map.entries() {
        let length = descriptor.page_count.checked_mul(PAGE_SIZE as u64)?;

        let (kind, attributes) = translate_memory_type(descriptor.ty);

        let region = MemoryRegion::new(descriptor.phys_start, length, kind, attributes);

        if region.is_empty() {
            continue;
        }

        if count >= capacity {
            return None;
        }

        unsafe {
            ptr::write(output.add(count), region);
        }

        count += 1;
    }

    Some(count)
}

unsafe fn sort_regions(regions: *mut MemoryRegion, count: usize) {
    for index in 1..count {
        let current = unsafe { *regions.add(index) };

        let mut position = index;

        while position != 0 {
            let previous = unsafe { *regions.add(position - 1) };

            if previous.base <= current.base {
                break;
            }

            unsafe {
                ptr::write(regions.add(position), previous);
            }

            position -= 1;
        }

        unsafe {
            ptr::write(regions.add(position), current);
        }
    }
}

unsafe fn merge_adjacent(regions: *mut MemoryRegion, count: usize) -> Option<usize> {
    if count == 0 {
        return Some(0);
    }

    let mut write_index = 1usize;

    for read_index in 1..count {
        let current = unsafe { *regions.add(read_index) };

        let previous = unsafe { &mut *regions.add(write_index - 1) };

        if previous.kind == current.kind
            && previous.attributes == current.attributes
            && previous.end() == Some(current.base)
        {
            previous.length = previous.length.checked_add(current.length)?;

            continue;
        }

        unsafe {
            ptr::write(regions.add(write_index), current);
        }

        write_index += 1;
    }

    Some(write_index)
}

unsafe fn reserve_range(
    regions: *mut MemoryRegion,
    mut count: usize,
    capacity: usize,
    reservation: MemoryRegion,
) -> Option<usize> {
    let reservation_end = reservation.end()?;

    if reservation.is_empty() {
        return None;
    }

    let mut covered = 0u64;

    let mut index = 0usize;

    while index < count {
        let current = unsafe { *regions.add(index) };

        let current_end = current.end()?;

        if current_end <= reservation.base {
            index += 1;
            continue;
        }

        if current.base >= reservation_end {
            break;
        }

        let overlap_start = current.base.max(reservation.base);

        let overlap_end = current_end.min(reservation_end);

        if overlap_start >= overlap_end {
            index += 1;
            continue;
        }

        if current.kind != MemoryKind::USABLE && current.kind != MemoryKind::BOOT_RECLAIMABLE {
            return None;
        }

        let before_length = overlap_start - current.base;

        let reserved_length = overlap_end - overlap_start;

        let after_length = current_end - overlap_end;

        let mut pieces = 1usize;

        if before_length != 0 {
            pieces += 1;
        }

        if after_length != 0 {
            pieces += 1;
        }

        let extra = pieces - 1;

        if count.checked_add(extra)? > capacity {
            return None;
        }

        let tail = count - index - 1;

        if extra != 0 && tail != 0 {
            unsafe {
                ptr::copy(regions.add(index + 1), regions.add(index + pieces), tail);
            }
        }

        let mut write = index;

        if before_length != 0 {
            unsafe {
                ptr::write(
                    regions.add(write),
                    MemoryRegion::new(
                        current.base,
                        before_length,
                        current.kind,
                        current.attributes,
                    ),
                );
            }

            write += 1;
        }

        unsafe {
            ptr::write(
                regions.add(write),
                MemoryRegion::new(
                    overlap_start,
                    reserved_length,
                    reservation.kind,
                    reservation.attributes,
                ),
            );
        }

        write += 1;

        if after_length != 0 {
            unsafe {
                ptr::write(
                    regions.add(write),
                    MemoryRegion::new(overlap_end, after_length, current.kind, current.attributes),
                );
            }
        }

        count += extra;

        covered = covered.checked_add(reserved_length)?;

        index += pieces;
    }

    if covered != reservation.length {
        return None;
    }

    Some(count)
}

unsafe fn validate_non_overlapping(regions: *const MemoryRegion, count: usize) -> bool {
    let mut previous_end = 0u64;

    let mut have_previous = false;

    for index in 0..count {
        let region = unsafe { *regions.add(index) };

        if region.is_empty() {
            return false;
        }

        let Some(end) = region.end() else {
            return false;
        };

        if have_previous && region.base < previous_end {
            return false;
        }

        previous_end = end;

        have_previous = true;
    }

    true
}

unsafe fn range_is_kind(
    regions: *const MemoryRegion,
    count: usize,
    base: u64,
    length: u64,
    kind: MemoryKind,
) -> bool {
    if length == 0 {
        return false;
    }

    let Some(end) = base.checked_add(length) else {
        return false;
    };

    let mut cursor = base;

    for index in 0..count {
        let region = unsafe { *regions.add(index) };

        let Some(region_end) = region.end() else {
            return false;
        };

        if region_end <= cursor {
            continue;
        }

        if region.base > cursor {
            return false;
        }

        if region.kind != kind {
            return false;
        }

        cursor = region_end.min(end);

        if cursor == end {
            return true;
        }
    }

    false
}

fn kernel_attributes() -> MemoryAttributes {
    MemoryAttributes::CPU_ADDRESSABLE
        .union(MemoryAttributes::READABLE)
        .union(MemoryAttributes::WRITABLE)
        .union(MemoryAttributes::EXECUTABLE)
        .union(MemoryAttributes::VOLATILE)
}

fn boot_info_attributes() -> MemoryAttributes {
    MemoryAttributes::CPU_ADDRESSABLE
        .union(MemoryAttributes::READABLE)
        .union(MemoryAttributes::WRITABLE)
        .union(MemoryAttributes::VOLATILE)
}

fn translate_memory_type(ty: MemoryType) -> (MemoryKind, MemoryAttributes) {
    let cpu_rw_volatile = MemoryAttributes::CPU_ADDRESSABLE
        .union(MemoryAttributes::READABLE)
        .union(MemoryAttributes::WRITABLE)
        .union(MemoryAttributes::VOLATILE);

    let boot_reclaimable = MemoryAttributes::CPU_ADDRESSABLE.union(MemoryAttributes::RECLAIMABLE);

    let acpi_reclaimable = MemoryAttributes::CPU_ADDRESSABLE
        .union(MemoryAttributes::READABLE)
        .union(MemoryAttributes::RECLAIMABLE);

    let device = MemoryAttributes::CPU_ADDRESSABLE.union(MemoryAttributes::DEVICE);

    let persistent = MemoryAttributes::CPU_ADDRESSABLE
        .union(MemoryAttributes::READABLE)
        .union(MemoryAttributes::WRITABLE)
        .union(MemoryAttributes::PERSISTENT);

    if ty == MemoryType::CONVENTIONAL {
        (MemoryKind::USABLE, cpu_rw_volatile)
    } else if ty == MemoryType::LOADER_CODE
        || ty == MemoryType::LOADER_DATA
        || ty == MemoryType::BOOT_SERVICES_CODE
        || ty == MemoryType::BOOT_SERVICES_DATA
    {
        (MemoryKind::BOOT_RECLAIMABLE, boot_reclaimable)
    } else if ty == MemoryType::RUNTIME_SERVICES_CODE || ty == MemoryType::RUNTIME_SERVICES_DATA {
        (
            MemoryKind::FIRMWARE_RUNTIME,
            MemoryAttributes::CPU_ADDRESSABLE,
        )
    } else if ty == MemoryType::ACPI_RECLAIM {
        (MemoryKind::ACPI_RECLAIMABLE, acpi_reclaimable)
    } else if ty == MemoryType::ACPI_NON_VOLATILE {
        (
            MemoryKind::ACPI_NVS,
            MemoryAttributes::CPU_ADDRESSABLE.union(MemoryAttributes::READABLE),
        )
    } else if ty == MemoryType::MMIO || ty == MemoryType::MMIO_PORT_SPACE {
        (MemoryKind::MMIO, device)
    } else if ty == MemoryType::PERSISTENT_MEMORY {
        (MemoryKind::PERSISTENT, persistent)
    } else if ty == MemoryType::RESERVED
        || ty == MemoryType::UNUSABLE
        || ty == MemoryType::PAL_CODE
        || ty == MemoryType::UNACCEPTED
    {
        (MemoryKind::RESERVED, MemoryAttributes::NONE)
    } else {
        (MemoryKind::UNKNOWN, MemoryAttributes::NONE)
    }
}
