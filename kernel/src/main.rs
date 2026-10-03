#![no_main]
#![no_std]

mod arch;

use core::{mem, panic::PanicInfo, ptr};

use clearfold_boot_abi::{BootInfo, MemoryKind, MemoryRegion};

use arch::x86_64::{disable_interrupts, halt_forever, serial};

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_entry(boot_info: *const BootInfo) -> ! {
    disable_interrupts();

    serial::init();

    serial::write_str("\n");
    serial::write_str("Clearfold Kernel\n");
    serial::write_str("================\n");
    serial::write_str("Computation, not Process.\n");
    serial::write_str("\n");

    serial::write_str("[CLEARFOLD KERNEL] architecture : x86_64\n");

    serial::write_str("[CLEARFOLD KERNEL] milestone    : P0/M1.2a\n");

    serial::write_str("[CLEARFOLD KERNEL] entry        : OK\n");

    serial::write_str("[CLEARFOLD KERNEL] UEFI services: GONE\n");

    if boot_info.is_null() {
        boot_failure("BootInfo pointer is null");
    }

    let info = unsafe { &*boot_info };

    validate_boot_info(info, boot_info);

    serial::write_fmt(format_args!(
        "[CLEARFOLD KERNEL] Boot ABI     : {}.{}\n",
        info.abi_major, info.abi_minor,
    ));

    serial::write_fmt(format_args!(
        "[CLEARFOLD KERNEL] BootInfo     : {:#018x}\n",
        boot_info as usize,
    ));

    serial::write_fmt(format_args!(
        "[CLEARFOLD KERNEL] kernel span  : {:#018x}..{:#018x}\n",
        info.kernel_phys_base,
        info.kernel_phys_base.saturating_add(info.kernel_phys_size,),
    ));

    serial::write_fmt(format_args!(
        "[CLEARFOLD KERNEL] memory regions: {}\n",
        info.memory_region_count,
    ));

    print_memory_map(info);

    serial::write_str("[CLEARFOLD KERNEL] memory map validation : OK\n");

    serial::write_str("[CLEARFOLD KERNEL] kernel ownership      : OK\n");

    serial::write_str("[CLEARFOLD KERNEL] BootInfo ownership    : OK\n");

    halt_forever()
}

fn validate_boot_info(info: &BootInfo, boot_info_ptr: *const BootInfo) {
    if !info.is_compatible() {
        boot_failure("incompatible Boot ABI");
    }

    if info.memory_region_count != 0 && info.memory_regions_ptr == 0 {
        boot_failure("memory map pointer is null");
    }

    let count = info.memory_region_count as usize;

    let stride = info.memory_region_entry_size as usize;

    let mut previous_end = 0u64;

    let mut have_previous = false;

    for index in 0..count {
        let region = memory_region_at(info, index);

        if region.is_empty() {
            boot_failure("empty memory region");
        }

        let end = match region.end() {
            Some(end) => end,
            None => {
                boot_failure("memory region overflow");
            }
        };

        if have_previous && region.base < previous_end {
            boot_failure("memory regions overlap or are unsorted");
        }

        previous_end = end;

        have_previous = true;
    }

    if !range_is_kind(
        info,
        info.kernel_phys_base,
        info.kernel_phys_size,
        MemoryKind::KERNEL,
    ) {
        boot_failure("kernel range is not kernel-owned");
    }

    if !range_is_kind(
        info,
        boot_info_ptr as u64,
        mem::size_of::<BootInfo>() as u64,
        MemoryKind::BOOT_INFO,
    ) {
        boot_failure("BootInfo is not boot-info-owned");
    }

    let map_bytes = match (count as u64).checked_mul(stride as u64) {
        Some(bytes) => bytes,
        None => {
            boot_failure("memory map size overflow");
        }
    };

    if map_bytes != 0
        && !range_is_kind(
            info,
            info.memory_regions_ptr,
            map_bytes,
            MemoryKind::BOOT_INFO,
        )
    {
        boot_failure("memory map is not boot-info-owned");
    }
}

fn range_is_kind(info: &BootInfo, base: u64, length: u64, expected_kind: MemoryKind) -> bool {
    if length == 0 {
        return false;
    }

    let Some(end) = base.checked_add(length) else {
        return false;
    };

    let mut cursor = base;

    for index in 0..info.memory_region_count as usize {
        let region = memory_region_at(info, index);

        let Some(region_end) = region.end() else {
            return false;
        };

        if region_end <= cursor {
            continue;
        }

        if region.base > cursor {
            return false;
        }

        if region.kind != expected_kind {
            return false;
        }

        cursor = region_end.min(end);

        if cursor == end {
            return true;
        }
    }

    false
}

fn memory_region_at(info: &BootInfo, index: usize) -> MemoryRegion {
    let stride = info.memory_region_entry_size as usize;

    let offset = match index.checked_mul(stride) {
        Some(offset) => offset,
        None => {
            boot_failure("memory map offset overflow");
        }
    };

    let region_ptr = unsafe {
        (info.memory_regions_ptr as *const u8)
            .add(offset)
            .cast::<MemoryRegion>()
    };

    unsafe { ptr::read_unaligned(region_ptr) }
}

fn print_memory_map(info: &BootInfo) {
    for index in 0..info.memory_region_count as usize {
        let region = memory_region_at(info, index);

        let end = region.end().unwrap_or(u64::MAX);

        serial::write_fmt(format_args!(
            "  [{:02}] {:<18} {:#018x}..{:#018x}  {} KiB\n",
            index,
            memory_kind_name(region.kind,),
            region.base,
            end,
            region.length / 1024,
        ));
    }
}

fn memory_kind_name(kind: MemoryKind) -> &'static str {
    if kind == MemoryKind::USABLE {
        "usable"
    } else if kind == MemoryKind::BOOT_RECLAIMABLE {
        "boot-reclaimable"
    } else if kind == MemoryKind::KERNEL {
        "kernel"
    } else if kind == MemoryKind::BOOT_INFO {
        "boot-info"
    } else if kind == MemoryKind::FIRMWARE_RUNTIME {
        "firmware-runtime"
    } else if kind == MemoryKind::ACPI_RECLAIMABLE {
        "acpi-reclaimable"
    } else if kind == MemoryKind::ACPI_NVS {
        "acpi-nvs"
    } else if kind == MemoryKind::MMIO {
        "mmio"
    } else if kind == MemoryKind::PERSISTENT {
        "persistent"
    } else if kind == MemoryKind::RESERVED {
        "reserved"
    } else {
        "unknown"
    }
}

fn boot_failure(message: &str) -> ! {
    serial::write_str("[CLEARFOLD KERNEL] BOOT FAILURE: ");

    serial::write_str(message);

    serial::write_str("\n");

    halt_forever()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial::init();

    serial::write_str("\n[CLEARFOLD KERNEL] PANIC\n");

    halt_forever()
}
