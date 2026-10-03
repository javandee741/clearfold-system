#![no_std]

use core::mem;

pub const BOOT_ABI_MAGIC: u64 = u64::from_le_bytes(*b"CFBOOT01");

pub const BOOT_ABI_MAJOR: u16 = 1;
pub const BOOT_ABI_MINOR: u16 = 0;

/// Information transferred from Clearfold Stage-0 to the kernel.
///
/// The structure is deliberately firmware-independent.
/// The kernel must never receive UEFI structures directly.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BootInfo {
    /// Identifies a Clearfold Boot ABI structure.
    pub magic: u64,

    /// Breaking ABI version.
    pub abi_major: u16,

    /// Backward-compatible ABI extension version.
    pub abi_minor: u16,

    /// Size of this BootInfo structure in bytes.
    pub struct_size: u32,

    /// Address of the first MemoryRegion entry.
    ///
    /// During early P0 this is identity-mapped.
    pub memory_regions_ptr: u64,

    /// Number of valid MemoryRegion entries.
    pub memory_region_count: u32,

    /// Size of one MemoryRegion entry.
    ///
    /// This allows future ABI versions to append fields.
    pub memory_region_entry_size: u32,

    /// Physical start of the loaded kernel image.
    pub kernel_phys_base: u64,

    /// Physical span reserved for the kernel image.
    pub kernel_phys_size: u64,

    /// Boot-wide feature/status flags.
    pub flags: u64,
}

impl BootInfo {
    pub const fn empty() -> Self {
        Self {
            magic: BOOT_ABI_MAGIC,
            abi_major: BOOT_ABI_MAJOR,
            abi_minor: BOOT_ABI_MINOR,
            struct_size: mem::size_of::<Self>() as u32,

            memory_regions_ptr: 0,
            memory_region_count: 0,
            memory_region_entry_size: mem::size_of::<MemoryRegion>() as u32,

            kernel_phys_base: 0,
            kernel_phys_size: 0,

            flags: 0,
        }
    }

    pub const fn has_valid_magic(&self) -> bool {
        self.magic == BOOT_ABI_MAGIC
    }

    pub const fn is_compatible(&self) -> bool {
        self.has_valid_magic()
            && self.abi_major == BOOT_ABI_MAJOR
            && self.struct_size >= mem::size_of::<Self>() as u32
            && self.memory_region_entry_size >= mem::size_of::<MemoryRegion>() as u32
    }
}

/// One physical address range visible during boot.
///
/// This abstraction is only for classical addressable memory.
/// It must not be used as a universal representation for GPU, QPU,
/// molecular, remote or other non-memory computational state.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryRegion {
    /// Physical base address.
    pub base: u64,

    /// Length in bytes.
    pub length: u64,

    /// Semantic type of this region.
    pub kind: MemoryKind,

    /// Reserved for ABI expansion/alignment.
    pub reserved: u32,

    /// Additional region properties.
    pub attributes: MemoryAttributes,
}

impl MemoryRegion {
    pub const fn new(
        base: u64,
        length: u64,
        kind: MemoryKind,
        attributes: MemoryAttributes,
    ) -> Self {
        Self {
            base,
            length,
            kind,
            reserved: 0,
            attributes,
        }
    }

    pub const fn end(&self) -> Option<u64> {
        self.base.checked_add(self.length)
    }

    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }
}

/// Stable, forward-compatible memory-region identifier.
///
/// This is a transparent integer rather than a Rust enum so that
/// future ABI versions may introduce values unknown to an older kernel
/// without creating an invalid Rust enum discriminant.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryKind(pub u32);

impl MemoryKind {
    pub const USABLE: Self = Self(1);

    /// Memory occupied during boot that may become reclaimable later.
    pub const BOOT_RECLAIMABLE: Self = Self(2);

    /// Memory reserved for the loaded Clearfold kernel.
    pub const KERNEL: Self = Self(3);

    /// Memory containing the BootInfo structure or boot ABI payload.
    pub const BOOT_INFO: Self = Self(4);

    /// Firmware runtime memory that must not be used as normal RAM.
    pub const FIRMWARE_RUNTIME: Self = Self(5);

    pub const ACPI_RECLAIMABLE: Self = Self(6);
    pub const ACPI_NVS: Self = Self(7);

    /// Device memory or memory-mapped I/O.
    pub const MMIO: Self = Self(8);

    pub const PERSISTENT: Self = Self(9);

    /// Unusable or intentionally reserved physical address space.
    pub const RESERVED: Self = Self(10);

    /// Region type not understood by the current ABI implementation.
    pub const UNKNOWN: Self = Self(u32::MAX);
}

/// Properties of a classical MemoryRegion.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryAttributes(pub u64);

impl MemoryAttributes {
    pub const NONE: Self = Self(0);

    pub const CPU_ADDRESSABLE: Self = Self(1 << 0);
    pub const READABLE: Self = Self(1 << 1);
    pub const WRITABLE: Self = Self(1 << 2);
    pub const EXECUTABLE: Self = Self(1 << 3);
    pub const VOLATILE: Self = Self(1 << 4);
    pub const PERSISTENT: Self = Self(1 << 5);
    pub const DEVICE: Self = Self(1 << 6);
    pub const RECLAIMABLE: Self = Self(1 << 7);

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_info_layout_is_stable() {
        assert_eq!(mem::size_of::<BootInfo>(), 56);
        assert_eq!(mem::align_of::<BootInfo>(), 8);
    }

    #[test]
    fn memory_region_layout_is_stable() {
        assert_eq!(mem::size_of::<MemoryRegion>(), 32);
        assert_eq!(mem::align_of::<MemoryRegion>(), 8);
    }

    #[test]
    fn empty_boot_info_is_compatible() {
        let info = BootInfo::empty();

        assert!(info.has_valid_magic());
        assert!(info.is_compatible());
    }

    #[test]
    fn memory_region_end_is_checked() {
        let normal = MemoryRegion::new(
            0x1000,
            0x2000,
            MemoryKind::USABLE,
            MemoryAttributes::CPU_ADDRESSABLE,
        );

        assert_eq!(normal.end(), Some(0x3000));

        let overflowing = MemoryRegion::new(
            u64::MAX - 1,
            4,
            MemoryKind::RESERVED,
            MemoryAttributes::NONE,
        );

        assert_eq!(overflowing.end(), None);
    }

    #[test]
    fn attributes_can_be_combined() {
        let attributes = MemoryAttributes::CPU_ADDRESSABLE
            .union(MemoryAttributes::READABLE)
            .union(MemoryAttributes::WRITABLE);

        assert!(attributes.contains(MemoryAttributes::CPU_ADDRESSABLE));
        assert!(attributes.contains(MemoryAttributes::READABLE));
        assert!(attributes.contains(MemoryAttributes::WRITABLE));
        assert!(!attributes.contains(MemoryAttributes::EXECUTABLE));
    }
}
