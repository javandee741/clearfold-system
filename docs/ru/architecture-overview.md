# Обзор архитектуры Clearfold

Clearfold строится вокруг малого capability-based микроядра, графа user-space сервисов и гетерогенного вычислительного слоя. Process, File, Socket, User и семантика GPU/QPU намеренно не являются фундаментальными объектами ядра.

| Версия | Модель |
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

Базовый словарь архитектуры: **Capability, MemoryRegion, AddressSpace, ExecutionContext, Endpoint, Object, Service, DataObject, Computation, ComputeGraph**.
