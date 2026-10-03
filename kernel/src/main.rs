#![no_main]
#![no_std]

mod arch;

use core::{panic::PanicInfo, ptr};

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

    serial::write_str("[CLEARFOLD KERNEL] milestone    : P0/M1.1\n");

    serial::write_str("[CLEARFOLD KERNEL] entry        : OK\n");

    serial::write_str("[CLEARFOLD KERNEL] UEFI services: GONE\n");

    if boot_info.is_null() {
        boot_failure("BootInfo pointer is null");
    }

    let info = unsafe { &*boot_info };

    if !info.is_compatible() {
        boot_failure("incompatible Boot ABI");
    }

    if info.memory_region_count != 0 && info.memory_regions_ptr == 0 {
        boot_failure("memory map pointer is null");
    }

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

    serial::write_str("[CLEARFOLD KERNEL] BootInfo validation: OK\n");

    halt_forever()
}

fn print_memory_map(info: &BootInfo) {
    let count = info.memory_region_count as usize;

    let stride = info.memory_region_entry_size as usize;

    for index in 0..count {
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

        // read_unaligned keeps future ABI versions safe even if their
        // entry stride differs from the current Rust alignment.
        let region = unsafe { ptr::read_unaligned(region_ptr) };

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
