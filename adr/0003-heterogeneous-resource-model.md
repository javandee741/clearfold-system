# ADR-0003: Heterogeneous resource and representation model

- Status: Accepted
- Date: 2026-10-03
- Affects: architecture, kernel, runtime, compute, boot ABI

## Context

Clearfold is intended to support heterogeneous computing systems whose
execution resources may have fundamentally different computational and
state models.

Examples include:

- classical CPUs;
- GPUs;
- NPUs and tensor accelerators;
- FPGAs;
- quantum processors;
- Ising and optimization accelerators;
- ternary or other non-binary accelerators;
- molecular, DNA or biological computing systems;
- remote computing resources;
- future computing substrates not known when the architecture is defined.

These systems cannot all be represented correctly as CPUs with another
kind of byte-addressable memory.

In particular:

- quantum state is not arbitrary copyable byte-addressable memory;
- measurement may transform or destroy quantum state;
- molecular state may be represented by populations, concentrations,
  sequences or reactions rather than addresses;
- GPU/device memory may have different visibility and transfer semantics;
- some embedded systems have fragmented physical memory, MPU-only
  protection, or no MMU;
- some systems execute code directly from persistent storage or flash.

A universal abstraction based on contiguous RAM would therefore create
a fundamental architectural limitation.

## Decision

Clearfold separates five concepts:

1. MemoryRegion
2. DataObject
3. Representation
4. Transformation
5. ComputeBackend

### MemoryRegion

MemoryRegion represents only classically addressable memory.

It describes physical or virtual memory that can meaningfully be
represented as address ranges.

MemoryRegion is not the universal representation of all computational
state.

### DataObject

DataObject represents logical information independently of the physical
substrate currently holding that information.

A DataObject may be represented by:

- ordinary RAM;
- persistent storage;
- GPU-local memory;
- remote storage;
- a quantum state;
- molecular or biological state;
- another future representation.

A DataObject therefore does not imply byte addressability.

### Representation

Representation describes how a logical DataObject exists on a specific
storage or computational substrate.

Different representations of the same logical data may have different
properties.

Examples:

- CPU byte array;
- GPU tensor;
- compressed object;
- quantum state;
- encoded DNA sequence.

### Transformation

Movement between representations is modeled as a Transformation.

A Transformation is not assumed to be a byte-for-byte copy.

Examples:

CPU → GPU:
    transfer or mapping

QPU → CPU:
    measurement

CPU → DNA:
    encoding + synthesis

DNA → CPU:
    retrieval + sequencing + decoding

Some transformations may be:

- destructive;
- probabilistic;
- lossy;
- expensive;
- irreversible;
- asynchronous.

### ComputeBackend

A ComputeBackend describes a computational substrate.

It advertises:

- supported operations;
- accepted input representations;
- produced output representations;
- resource requirements;
- latency and throughput properties;
- transfer requirements;
- reliability/error characteristics;
- architecture-specific constraints.

Applications and ComputeGraphs should describe required computation
rather than hard-code a physical execution device whenever possible.

The Compute Manager may select an appropriate backend according to
policy and cost.

### Resource semantics

Not every resource supports the same operations.

For example, a classical MemoryRegion may support:

- READ
- WRITE
- MAP
- COPY

while a future QuantumState resource may support:

- OPERATE
- MEASURE
- TRANSFER
- DESTROY

without supporting arbitrary COPY or MAP_AS_BYTES.

Capabilities authorize operations that are meaningful for the resource
type. They do not force all resource types into a common memory model.

### Classical physical memory

The Clearfold physical memory allocator must support multiple
discontiguous usable memory regions.

It must never require all usable physical memory to form one contiguous
range.

Physical contiguity is treated as a specific resource constraint rather
than the default allocation model.

### Platform profiles

Clearfold may support different memory-management profiles.

MMU profile:
- virtual memory;
- page tables;
- discontiguous physical pages;
- contiguous virtual mappings.

MPU profile:
- region-based protection;
- constrained number and shape of protected areas;
- partly static placement.

No-MMU profile:
- platform-specific static or semi-static placement;
- execution in place where appropriate;
- Forge-generated memory layout.

These profiles may share higher-level Clearfold abstractions without
requiring identical low-level memory mechanisms.

### Kernel boundary

The kernel remains mechanism-oriented.

The kernel is not required to understand:

- quantum algorithms;
- molecular chemistry;
- DNA encoding;
- GPU programming models;
- application-level Computation graphs.

Backend-specific policy and execution logic belongs in user-space
runtime and compute services unless a minimal kernel mechanism is
required for isolation or hardware access.

## Consequences

Positive consequences:

- Clearfold does not assume that all information is byte-addressable RAM.
- New computing substrates can be introduced without redefining the kernel.
- Fragmented embedded memory can be represented naturally.
- Computational placement can include representation conversion cost.
- Capability semantics can reflect physical properties of resources.
- Applications can remain less dependent on specific hardware.

Negative consequences:

- Runtime resource modeling becomes more complex.
- Transformations must be represented explicitly.
- Placement decisions require richer cost models.
- Not every DataObject operation can be universally available.
- Backend discovery and negotiation protocols will be required later.

## Alternatives considered

### Treat every accelerator as a device with memory

Rejected because quantum, molecular and future computational states may
not have ordinary memory semantics.

### Put heterogeneous computation semantics into the kernel

Rejected because it would make the kernel large, policy-heavy and tied
to particular generations of accelerators.

### Require one universal memory representation

Rejected because transformations such as measurement, synthesis and
sequencing are not ordinary memory copies.

## Compatibility / migration

This decision constrains future APIs but does not change the current P0
kernel ABI.

P0/M1 MemoryRegion will describe only classical physical memory.

DataObject, Representation, Transformation and ComputeBackend are
architectural concepts to be implemented incrementally in later runtime
milestones.

The M0 fixed physical kernel load address is a bootstrap mechanism only
and is not part of the permanent Clearfold memory model.
