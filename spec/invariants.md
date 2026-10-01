# Core invariants

- No operation on a kernel object succeeds without a valid capability carrying the required right.
- Capability derivation cannot increase rights or resource range.
- Address-space mappings require authority to the source MemoryRegion and target AddressSpace.
- W^X is the default memory policy.
- Kernel capabilities are local to one kernel instance and never cross a node boundary.
- Kernel IPC provides bounded rendezvous/events, not arbitrary user message queues.
- Computation, File, Directory, Socket, User and high-level service semantics are not kernel objects.
- Identity is not authority; actual authority is represented by capabilities.
- New domains receive only explicitly delegated initial authority.
