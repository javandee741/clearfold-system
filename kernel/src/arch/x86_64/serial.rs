use core::arch::asm;
use core::fmt::{self, Write};

const COM1: u16 = 0x3F8;

#[inline]
unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(
                nomem,
                nostack,
                preserves_flags,
            )
        );
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(
                nomem,
                nostack,
                preserves_flags,
            )
        );
    }

    value
}

pub fn init() {
    unsafe {
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x80);

        // Divisor 3 => 38400 baud.
        outb(COM1, 0x03);
        outb(COM1 + 1, 0x00);

        // 8 data bits, no parity, one stop bit.
        outb(COM1 + 3, 0x03);

        outb(COM1 + 2, 0xC7);
        outb(COM1 + 4, 0x0B);
    }
}

fn transmit_ready() -> bool {
    unsafe { inb(COM1 + 5) & 0x20 != 0 }
}

pub fn write_byte(byte: u8) {
    while !transmit_ready() {
        core::hint::spin_loop();
    }

    unsafe {
        outb(COM1, byte);
    }
}

fn write_raw(text: &str) {
    for byte in text.bytes() {
        if byte == b'\n' {
            write_byte(b'\r');
        }

        write_byte(byte);
    }
}

pub fn write_str(text: &str) {
    write_raw(text);
}

pub fn write_fmt(args: fmt::Arguments<'_>) {
    let mut writer = SerialWriter;

    let _ = writer.write_fmt(args);
}

struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        write_raw(text);
        Ok(())
    }
}
