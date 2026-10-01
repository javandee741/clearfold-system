# Clearfold architecture overview

Clearfold is organized around a small capability microkernel, a graph of user-space services, and a heterogeneous compute layer. It deliberately avoids making Process, File, Socket, User or GPU/QPU semantics kernel foundations.


| Version | Model | Core question |
|---|---|---|
| v0.1 | Kernel Architecture | What must remain privileged? |
| v0.2 | Kernel Object Model | What objects exist in the kernel? |
| v0.3 | Capability Model | How is authority represented and delegated? |
| v0.4 | Memory Model | How are memory ownership and mapping separated? |
| v0.5 | Execution & Scheduling | How is CPU execution separated from Computation? |
| v0.6 | IPC & Event Model | How do domains communicate without kernel queues? |
| v0.7 | Device & Driver Model | How are drivers isolated in user space? |
| v0.8 | Storage & Object Model | What replaces “file” as the fundamental persistent abstraction? |
| v0.9 | Service & Namespace Model | How is user space composed without global ambient authority? |
| v0.10 | Security, Identity & Policy | Who decides which capabilities are issued? |
| v0.11 | Application & Runtime Model | What is a native application in Clearfold? |
| v0.12 | Compute Graph & Heterogeneous Runtime | How are computations placed across unlike backends? |
| v0.13 | Distributed & Multi-Node | How do nodes cooperate without pretending remote == local? |
| v0.14 | Boot & Meta-System | How is a specialized runtime constructed and updated? |
| v0.15 | Formal Spec & Prototype Boundary | What exact subset is implemented first? |


## Architectural vocabulary

The foundational terms are **Capability**, **MemoryRegion**, **AddressSpace**, **ExecutionContext**, **Endpoint**, **Object**, **Service**, **DataObject**, **Computation** and **ComputeGraph**.

Traditional Process/File/Socket semantics may exist in compatibility services without changing the native model.
