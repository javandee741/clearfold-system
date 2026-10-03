use core::{mem, ptr};

use clearfold_boot_abi::{BootInfo, MemoryAttributes, MemoryKind, MemoryRegion};

use uefi::boot::{self, AllocateType, MemoryType};
use uefi::mem::memory_map::MemoryMap;

const PAGE_SIZE: usize = 4096;

/// Extra descriptor capacity reserved because allocations performed
/// between a probe memory map and ExitBootServices may split existing
/// UEFI descriptors.
const MEMORY_MAP_SLACK: usize = 64;

pub struct BootPayload {
    info: *mut BootInfo,
    regions: *mut MemoryRegion,
    capacity: usize,
}

impl BootPayload {
    /// Reserve memory for the Clearfold Boot ABI while UEFI Boot Services
    /// are still available.
    pub fn allocate() -> Self {
        let probe =
            boot::memory_map(MemoryType::LOADER_DATA).expect("cannot probe UEFI memory map");

        let descriptor_count = probe.entries().count();

        drop(probe);

        // P0 deliberately keeps generous headroom because subsequent
        // allocations may cause UEFI to split descriptors.
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

        unsafe {
            // The entire handoff area starts in a known state.
            ptr::write_bytes(base, 0, pages * PAGE_SIZE);
        }

        let info = base.cast::<BootInfo>();

        let regions = unsafe { base.add(regions_offset) }.cast::<MemoryRegion>();

        Self {
            info,
            regions,
            capacity,
        }
    }

    /// Convert the final firmware memory map into Clearfold MemoryRegion
    /// entries.
    ///
    /// This function is called after ExitBootServices and therefore MUST
    /// NOT allocate memory or invoke any UEFI Boot Service.
    pub unsafe fn finalize<M: MemoryMap>(
        &mut self,
        memory_map: &M,
        kernel_phys_base: u64,
        kernel_phys_size: u64,
    ) -> Option<*const BootInfo> {
        let region_count =
            unsafe { normalize_memory_map(memory_map, self.regions, self.capacity) }?;

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

        if !unsafe { append_or_merge(output, &mut count, capacity, region) } {
            return None;
        }
    }

    Some(count)
}

unsafe fn append_or_merge(
    output: *mut MemoryRegion,
    count: &mut usize,
    capacity: usize,
    region: MemoryRegion,
) -> bool {
    if *count != 0 {
        let previous_ptr = unsafe { output.add(*count - 1) };

        let previous = unsafe { &mut *previous_ptr };

        if previous.kind == region.kind
            && previous.attributes == region.attributes
            && previous.end() == Some(region.base)
        {
            if let Some(length) = previous.length.checked_add(region.length) {
                previous.length = length;
                return true;
            }

            return false;
        }
    }

    if *count >= capacity {
        return false;
    }

    unsafe {
        ptr::write(output.add(*count), region);
    }

    *count += 1;

    true
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
