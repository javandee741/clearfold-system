#![no_main]
#![no_std]

extern crate alloc;

mod bootinfo;

use alloc::vec::Vec;
use core::{arch::asm, mem, ptr};

use bootinfo::BootPayload;

use clearfold_boot_abi::BootInfo;

use elf::ElfBytes;
use elf::abi::{EM_X86_64, ET_EXEC, PT_LOAD};
use elf::endian::AnyEndian;

use uefi::boot::{self, AllocateType, MemoryType};
use uefi::fs::FileSystem;
use uefi::mem::memory_map::MemoryMap;
use uefi::prelude::*;

const PAGE_SIZE: usize = 4096;

struct LoadedKernel {
    entry: u64,
    phys_base: u64,
    phys_size: u64,
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().expect("UEFI helper initialization failed");

    uefi::println!();
    uefi::println!("Clearfold Stage-0");
    uefi::println!("=================");
    uefi::println!("Computation, not Process.");
    uefi::println!();

    uefi::println!("[CLEARFOLD] architecture : x86_64");
    uefi::println!("[CLEARFOLD] milestone    : P0/M1.1");
    uefi::println!("[CLEARFOLD] UEFI entry   : OK");

    let kernel = load_kernel();

    uefi::println!("[CLEARFOLD] kernel ELF   : OK");
    uefi::println!("[CLEARFOLD] kernel entry : {:#018x}", kernel.entry);

    let mut boot_payload = BootPayload::allocate();

    uefi::println!("[CLEARFOLD] Boot ABI     : prepared");

    uefi::println!("[CLEARFOLD] ExitBootServices...");

    // SAFETY:
    // All filesystem and protocol objects created by load_kernel()
    // are already gone. After this call no UEFI Boot Service or
    // UEFI-backed allocator may be used.
    let memory_map = unsafe { boot::exit_boot_services(None) };

    // No allocations are permitted beyond this point.

    let boot_info =
        match unsafe { boot_payload.finalize(&memory_map, kernel.phys_base, kernel.phys_size) } {
            Some(info) => info,
            None => halt_after_boot_services(),
        };

    let kernel_entry: extern "sysv64" fn(*const BootInfo) -> ! =
        unsafe { mem::transmute(kernel.entry as usize) };

    kernel_entry(boot_info)
}

fn load_kernel() -> LoadedKernel {
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

    let mut kernel_phys_base = u64::MAX;
    let mut kernel_phys_end = 0u64;

    for segment in segments.iter().filter(|segment| segment.p_type == PT_LOAD) {
        assert!(
            segment.p_memsz >= segment.p_filesz,
            "ELF PT_LOAD mem size is smaller than file size"
        );

        // P0 still deliberately uses identity mapping.
        assert_eq!(
            segment.p_vaddr, segment.p_paddr,
            "P0 requires vaddr == paddr"
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
            ptr::write_bytes(destination.as_ptr(), 0, pages * PAGE_SIZE);

            ptr::copy_nonoverlapping(file_data.as_ptr(), destination.as_ptr(), file_data.len());
        }

        let allocated_end = segment
            .p_paddr
            .checked_add((pages * PAGE_SIZE) as u64)
            .expect("kernel physical range overflow");

        kernel_phys_base = kernel_phys_base.min(segment.p_paddr);

        kernel_phys_end = kernel_phys_end.max(allocated_end);

        uefi::println!(
            "[CLEARFOLD] LOAD          {:#018x}..{:#018x}",
            segment.p_paddr,
            allocated_end
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

    LoadedKernel {
        entry,
        phys_base: kernel_phys_base,
        phys_size: kernel_phys_end - kernel_phys_base,
    }
}

fn dump_memory_around(address: u64, size: u64) {
    let map = boot::memory_map(MemoryType::LOADER_DATA).expect("cannot read UEFI memory map");

    let requested_end = address.saturating_add(size);

    let nearby_start = address.saturating_sub(0x0100_0000);

    let nearby_end = address.saturating_add(0x0100_0000);

    uefi::println!(
        "[CLEARFOLD] requested     : {:#018x}..{:#018x}",
        address,
        requested_end
    );

    for descriptor in map.entries() {
        let start = descriptor.phys_start;

        let end = start.saturating_add(descriptor.page_count * PAGE_SIZE as u64);

        let overlaps = address < end && requested_end > start;

        let nearby = start < nearby_end && end > nearby_start;

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

fn halt_after_boot_services() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack,));
        }
    }
}
