#![no_main]
#![no_std]

use core::time::Duration;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().expect("UEFI helper initialization failed");

    uefi::println!();
    uefi::println!("Clearfold Stage-0");
    uefi::println!("=================");
    uefi::println!("Computation, not Process.");
    uefi::println!();
    uefi::println!("[CLEARFOLD] architecture : x86_64");
    uefi::println!("[CLEARFOLD] milestone    : P0/M0a");
    uefi::println!("[CLEARFOLD] UEFI entry   : OK");

    // Keep the message visible long enough for the smoke test.
    uefi::boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}
