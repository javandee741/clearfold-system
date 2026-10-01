# Clearfold 架构概览

Clearfold 由小型 capability 微内核、用户态服务图和异构计算层组成。Process、File、Socket、User 以及 GPU/QPU 语义并不是内核基础对象。

| 版本 | 模型 |
|---|---|
| v0.1 | Kernel Architecture |
| v0.2 | Kernel Object Model |
| v0.3 | Capability Model |
| v0.4 | Memory Model |
| v0.5 | Execution & Scheduling Model |
| v0.6 | IPC & Event Model |
| v0.7 | Device & Driver Model |
| v0.8 | Storage & Object Model |
| v0.9 | Service & Namespace Model |
| v0.10 | Security, Identity & Policy Model |
| v0.11 | Application & Runtime Model |
| v0.12 | Compute Graph & Heterogeneous Runtime Model |
| v0.13 | Distributed & Multi-Node Model |
| v0.14 | Boot, Meta-System & System Construction Model |
| v0.15 | Formal System Specification & Minimal Prototype Boundary |

核心术语包括 **Capability、MemoryRegion、AddressSpace、ExecutionContext、Endpoint、Object、Service、DataObject、Computation、ComputeGraph**。
