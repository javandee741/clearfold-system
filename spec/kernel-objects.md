# Kernel objects — v0.15 baseline

Conceptual public kernel object types:

1. ResourcePool
2. CapabilitySpace
3. MemoryRegion
4. AddressSpace
5. ExecutionContext
6. SchedulingContext
7. Endpoint
8. Notification
9. Timer
10. Interrupt
11. DeviceRegion
12. DMAContext

Prototype P0 initially implements ResourcePool, CapabilitySpace, MemoryRegion, AddressSpace, ExecutionContext, SchedulingContext, Endpoint, Notification and Timer. Interrupt becomes a public object in a later device milestone; DeviceRegion and DMAContext follow with device support.
