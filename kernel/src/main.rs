#![no_main]
#![no_std]

mod arch;

use core::panic::PanicInfo;

use arch::x86_64::{halt_forever, serial};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_entry() -> ! {
    serial::init();

    serial::write_str("\n");
    serial::write_str("Clearfold Kernel\n");
    serial::write_str("================\n");
    serial::write_str("Computation, not Process.\n");
    serial::write_str("\n");
    serial::write_str("[CLEARFOLD KERNEL] architecture : x86_64\n");
    serial::write_str("[CLEARFOLD KERNEL] milestone    : P0/M0b\n");
    serial::write_str("[CLEARFOLD KERNEL] entry        : OK\n");

    halt_forever()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial::init();
    serial::write_str("\n[CLEARFOLD KERNEL] PANIC\n");

    halt_forever()
}
