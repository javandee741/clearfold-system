# Clearfold Boot ABI

Status: experimental P0/M1

## Purpose

The Boot ABI is the firmware-independent contract between Clearfold
Stage-0 and the Clearfold Kernel.

Firmware-specific structures must terminate at the Stage-0 boundary.

The kernel must not depend on UEFI, device tree, BIOS or another firmware
representation merely because Stage-0 used that mechanism to discover
the machine.

## Boundary

Platform / firmware
        |
        v
     Stage-0
        |
        | normalize
        v
Clearfold Boot ABI
        |
        v
      Kernel

## BootInfo

BootInfo is a versioned, C-layout structure.

The structure contains:

- ABI identity and version;
- normalized physical memory map location;
- normalized memory-region count;
- memory-region ABI size;
- loaded kernel physical range;
- boot feature flags.

Future ABI revisions append fields rather than changing the meaning or
offset of existing fields whenever possible.

Breaking changes increment the major ABI version.

## MemoryRegion

MemoryRegion represents one classical physical address range.

Memory maps may contain any number of discontiguous regions.

The kernel must not assume:

- that usable memory starts at a fixed address;
- that usable RAM is contiguous;
- that one sufficiently large physical span exists;
- that firmware memory layout is identical between boots;
- that desktop/server memory assumptions apply to embedded systems.

## Physical contiguity

Physical contiguity is a requested property, not the default memory
model.

Normal page allocation may be satisfied from independent physical
frames belonging to different usable regions.

A later virtual-memory subsystem may map discontiguous physical pages
into contiguous virtual ranges.

Resources that require true physical contiguity must request it
explicitly.

## Scope

MemoryRegion models only classically addressable memory.

It is not used to represent arbitrary computational state such as:

- quantum state;
- molecular state;
- accelerator-internal opaque state;
- remote logical objects.

Those belong to higher-level resource and representation abstractions.

See ADR-0003.
