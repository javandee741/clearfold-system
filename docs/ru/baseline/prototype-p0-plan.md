<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Prototype P0 — план реализации**

CLEARFOLD-P0-001 • версия 0.1

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                       |
|------------|------------|------------|-------------------------------------|
| 0.1        | 01.10.2026 | Baseline   | Implementation plan на основе v0.15 |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Цель P0

Получить минимально жизнеспособный x86-64/UEFI microkernel prototype,
который доказывает работоспособность фундаментальных решений Clearfold без
преждевременной разработки драйверов, filesystem, network и GUI.

# 2. Технологический baseline

| **Область**          | **Решение**                                      |
|----------------------|--------------------------------------------------|
| ISA                  | x86-64                                           |
| Firmware             | UEFI                                             |
| Основной язык        | Rust no_std                                      |
| Assembly             | boot/syscall/interrupt/context switch/AP startup |
| Страницы             | 4 KiB P0                                         |
| CPU                  | 1 CPU -\> 2–4 CPU SMP                            |
| Executable container | Ограниченный ELF subset временно                 |
| Debug output         | Serial/debug console                             |
| Build                | Cargo + custom target/linker script              |
| Emulation            | QEMU/OVMF прежде физического hardware            |

# 3. Структура репозитория

/system  
/spec  
/kernel/core  
/kernel/arch/x86_64  
/stage0  
/runtime/root-manager  
/runtime/pager  
/runtime/test-services  
/libs/abi  
/libs/ipc  
/tools  
/tests

# 4. Milestones

| **Milestone** | **Тема**             | **Definition of done**                                                   |
|---------------|----------------------|--------------------------------------------------------------------------|
| M0            | Boot                 | UEFI -\> kernel -\> serial; Boot Manifest minimal.                       |
| M1            | Memory               | Physical allocator, page tables, CPL3 entry, isolated AddressSpace.      |
| M2            | Objects/Capabilities | ResourcePool, CSpace, MemoryRegion, invoke, rights checks.               |
| M3            | Execution            | ExecutionContext, context switch, cooperative then preemptive scheduler. |
| M4            | IPC                  | Endpoint send/recv/call/reply_wait; echo benchmark.                      |
| M5            | Fault/Pager          | Page fault IPC; user-space pager maps page and resumes.                  |
| M6            | Timer/Preemption     | Notification, one-shot Timer, SchedulingContext budget.                  |
| M7            | SMP                  | AP startup, per-CPU scheduler data, IPI/TLB shootdown.                   |
| M8            | Cap transfer/revoke  | COPY/MINT/MOVE/DELETE/REVOKE + IPC capability transfer.                  |

# 5. P0 non-goals

- Filesystem и persistent storage.

- Network stack.

- USB/PCIe/NVMe user drivers до завершения M8.

- GPU/QPU backend.

- GUI.

- POSIX/Linux compatibility.

- Swap, NUMA policy, power management.

- Distributed runtime и Meta-System compiler.

# 6. Обязательные demo scenarios

1.  Echo RPC между изолированными domains.

2.  User-space Pager обрабатывает demand page fault.

3.  Rights attenuation: RW -\> R; WRITE map отклоняется.

4.  Forged CapHandle не вызывает kernel crash.

5.  Atomic MOVE authority между domains.

6.  Timer preemption переключает два CPU contexts.

7.  Faulted test domain уничтожается/локализуется без kernel panic.

# 7. Performance measurements

| **Метрика**           | **Метод**                                      |
|-----------------------|------------------------------------------------|
| syscall invoke cycles | rdtsc/architectural counter вокруг no-op query |
| IPC call RTT          | client/server ping-pong                        |
| context switch        | paired execution contexts                      |
| page fault delivery   | fault -\> pager -\> resume                     |
| cap lookup            | random/contiguous slots                        |
| TLB shootdown         | multi-CPU mapping revoke                       |

# 8. Coding constraints

- unsafe Rust концентрируется в arch/hardware/low-level memory modules.

- Kernel user input считается hostile; все sizes/offsets/handles
  проверяются.

- Kernel paths не должны выполнять unbounded loops по user-controlled
  cardinality без budget/incremental design.

- No hidden user-resource heap after bootstrap; resources accounted via
  ResourcePool model.

- Debug-only facilities не становятся обязательным production ABI.

# 9. Риски P0

| **Риск**                                    | **Снижение**                                                        |
|---------------------------------------------|---------------------------------------------------------------------|
| Слишком ранняя SMP-сложность                | Сначала single-core correctness; SMP только M7.                     |
| Capability revocation races                 | M8 после стабильных IPC/scheduler; отдельные concurrency tests.     |
| ABI преждевременно заморожен                | Freeze semantics, а не внутренние layouts; ADR для изменений.       |
| Rust unsafe разрастается                    | Module-level unsafe policy + review checklist.                      |
| Performance optimization ломает correctness | Correctness-first milestones; benchmark после работающей семантики. |
