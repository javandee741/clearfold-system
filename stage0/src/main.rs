#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::{mem, ptr};

use elf::ElfBytes;
use elf::abi::{EM_X86_64, ET_EXEC, PT_LOAD};
use elf::endian::AnyEndian;

use uefi::boot::{self, AllocateType, MemoryType};
use uefi::fs::FileSystem;
use uefi::prelude::*;

use uefi::mem::memory_map::MemoryMap;

const PAGE_SIZE: usize = 4096;

#[entry]
fn main() -> Status {
    uefi::helpers::init().expect("UEFI helper initialization failed");

    uefi::println!();
    uefi::println!("Clearfold Stage-0");
    uefi::println!("=================");
    uefi::println!("Computation, not Process.");
    uefi::println!();

    uefi::println!("[CLEARFOLD] architecture : x86_64");
    uefi::println!("[CLEARFOLD] milestone    : P0/M0b");
    uefi::println!("[CLEARFOLD] UEFI entry   : OK");

    let entry = load_kernel();

    uefi::println!("[CLEARFOLD] kernel ELF   : OK");
    uefi::println!("[CLEARFOLD] kernel entry : {entry:#018x}");
    uefi::println!("[CLEARFOLD] ExitBootServices...");

    // SAFETY:
    // All UEFI filesystem/protocol objects created by load_kernel()
    // have already been dropped. After this call Stage-0 must never
    // use UEFI Boot Services again.
    let _memory_map = unsafe { boot::exit_boot_services(None) };

    // From this point onward:
    //     no uefi::println!
    //     no UEFI allocator
    //     no UEFI filesystem/protocol calls

    let kernel_entry: extern "sysv64" fn() -> ! = unsafe { mem::transmute(entry as usize) };

    kernel_entry()
}

fn load_kernel() -> u64 {
    let kernel_bytes: Vec<u8> = {
        let fs_protocol =
            boot::get_image_file_system(boot::image_handle()).expect("cannot open boot filesystem");

        let mut fs = FileSystem::new(fs_protocol);

        fs.read(uefi::cstr16!("\\KERNEL.ELF"))
            .expect("cannot read KERNEL.ELF")
    };

    uefi::println!("[CLEARFOLD] kernel size  : {} bytes", kernel_bytes.len());

    let elf = ElfBytes::<AnyEndian>::minimal_parse(&kernel_bytes).expect("invalid kernel ELF");

    assert_eq!(elf.ehdr.e_machine, EM_X86_64, "kernel ELF is not x86_64");

    assert_eq!(elf.ehdr.e_type, ET_EXEC, "kernel ELF is not ET_EXEC");

    let entry = elf.ehdr.e_entry;

    let segments = elf.segments().expect("kernel ELF has no program headers");

    let mut loaded_segments = 0usize;
    let mut entry_is_loaded = false;

    for segment in segments.iter().filter(|segment| segment.p_type == PT_LOAD) {
        assert!(
            segment.p_memsz >= segment.p_filesz,
            "ELF PT_LOAD mem size is smaller than file size"
        );

        // M0b deliberately uses identity mapping.
        assert_eq!(
            segment.p_vaddr, segment.p_paddr,
            "M0b requires vaddr == paddr"
        );

        assert_eq!(
            segment.p_paddr % PAGE_SIZE as u64,
            0,
            "PT_LOAD address must be page aligned"
        );

        let memory_size = usize::try_from(segment.p_memsz).expect("PT_LOAD is too large");

        let pages = memory_size.div_ceil(PAGE_SIZE);

        dump_memory_around(segment.p_paddr, segment.p_memsz);

        let destination = boot::allocate_pages(
            AllocateType::Address(segment.p_paddr),
            MemoryType::LOADER_DATA,
            pages,
        )
        .expect("cannot allocate pages for kernel");

        assert_eq!(
            destination.as_ptr() as u64,
            segment.p_paddr,
            "UEFI allocated kernel at unexpected address"
        );

        let file_data = elf
            .segment_data(&segment)
            .expect("invalid PT_LOAD file range");

        unsafe {
            // First zero the in-memory segment. This also handles future .bss.
            ptr::write_bytes(destination.as_ptr(), 0, memory_size);

            // Then copy the bytes which actually exist in the ELF file.
            ptr::copy_nonoverlapping(file_data.as_ptr(), destination.as_ptr(), file_data.len());
        }

        uefi::println!(
            "[CLEARFOLD] LOAD          {:#018x}..{:#018x}",
            segment.p_paddr,
            segment.p_paddr + segment.p_memsz
        );

        if entry >= segment.p_vaddr && entry < segment.p_vaddr + segment.p_memsz {
            entry_is_loaded = true;
        }

        loaded_segments += 1;
    }

    assert!(
        loaded_segments != 0,
        "kernel ELF contains no PT_LOAD segments"
    );

    assert!(entry_is_loaded, "kernel entry is outside loaded segments");

    entry
}

fn dump_memory_around(address: u64, size: u64) {
    let map = boot::memory_map(MemoryType::LOADER_DATA).expect("cannot read UEFI memory map");

    let requested_end = address + size;

    uefi::println!(
        "[CLEARFOLD] requested     : {:#018x}..{:#018x}",
        address,
        requested_end
    );

    for descriptor in map.entries() {
        let start = descriptor.phys_start;
        let end = start + descriptor.page_count * PAGE_SIZE as u64;

        // Show the descriptor containing or touching our requested range,
        // plus a useful neighbourhood around 16 MiB.
        let overlaps = address < end && requested_end > start;

        let nearby = start < 0x0200_0000 && end > 0x0080_0000;

        if overlaps || nearby {
            uefi::println!(
                "[CLEARFOLD] mem {:?} {:#018x}..{:#018x} pages={}",
                descriptor.ty,
                start,
                end,
                descriptor.page_count
            );
        }
    }
}
