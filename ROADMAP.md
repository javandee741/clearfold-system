# Clearfold Roadmap

## Architecture baseline — complete

v0.1 through v0.15 define the kernel boundary, object model, capabilities, memory, execution, IPC, drivers, storage, services, security, applications, heterogeneous compute, multi-node operation, Meta-System and prototype boundary.

## Prototype P0 — next

- M0: UEFI → kernel → serial console
- M1: physical memory + page tables + user mode
- M2: kernel objects + capability spaces + `invoke`
- M3: execution contexts + basic scheduler
- M4: endpoint IPC (`send/recv/call/reply_wait`)
- M5: fault delivery + user-space pager
- M6: notifications, timers and preemption
- M7: SMP + TLB shootdown
- M8: capability COPY/MINT/MOVE/REVOKE + IPC transfer

## Prototype P1

Device primitives, interrupt objects, DMA/IOMMU model, PCI discovery and first user-space device driver.

## Prototype P2

NVMe/block service, object storage bootstrap and immutable system objects.

Later phases progressively introduce service management, security policy, network, graphical stack, heterogeneous Compute Graph backends and distributed execution.
