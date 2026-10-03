use core::arch::asm;

pub mod serial;

pub fn halt_forever() -> ! {
    loop {
        // SAFETY:
        // M0b executes in ring 0 and deliberately halts
        // until the next interrupt.
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
