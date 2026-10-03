use core::arch::asm;

pub mod serial;

pub fn disable_interrupts() {
    // SAFETY:
    // Clearfold M0b executes at CPL0.
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
}

pub fn halt_forever() -> ! {
    disable_interrupts();

    loop {
        // SAFETY:
        // Interrupts are disabled and the kernel intentionally stops here.
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
