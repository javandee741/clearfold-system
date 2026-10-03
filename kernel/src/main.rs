#![no_main]
#![no_std]

mod arch;

use core::panic::PanicInfo;

use arch::x86_64::{disable_interrupts, halt_forever, serial};

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_entry() -> ! {
    disable_interrupts();

    serial::init();

    serial::write_str("\n");
    serial::write_str("Clearfold Kernel\n");
    serial::write_str("================\n");
    serial::write_str("Computation, not Process.\n");
    serial::write_str("\n");
    serial::write_str("[CLEARFOLD KERNEL] architecture : x86_64\n");
    serial::write_str("[CLEARFOLD KERNEL] milestone    : P0/M0b\n");
    serial::write_str("[CLEARFOLD KERNEL] entry        : OK\n");
    serial::write_str("[CLEARFOLD KERNEL] UEFI services: GONE\n");

    halt_forever()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial::init();
    serial::write_str("\n[CLEARFOLD KERNEL] PANIC\n");

    halt_forever()
}
