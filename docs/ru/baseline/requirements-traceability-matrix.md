<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Requirements Traceability Matrix**

CLEARFOLD-RTM-001 • версия 0.1

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                                                  |
|------------|------------|------------|----------------------------------------------------------------|
| 0.1        | 01.10.2026 | Baseline   | Связь архитектурных baseline, документов, компонентов и тестов |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Назначение

Матрица трассируемости предотвращает потерю архитектурных решений при
переходе от обсуждения к коду. Каждое критическое требование должно
иметь источник, ответственную спецификацию, будущий компонент и способ
проверки.

# 2. Traceability v0.1–v0.15

| **Источник** | **Ключевое требование**                           | **Документ**         | **Компонент**               | **Проверка**                           |
|--------------|---------------------------------------------------|----------------------|-----------------------------|----------------------------------------|
| v0.1         | Kernel=mechanism not policy                       | ARCH §3 / TZ §2      | kernel/core                 | architecture review                    |
| v0.2         | 12 conceptual kernel object types                 | ARCH §4 / ABI §2     | kernel/object               | object registry tests                  |
| v0.3         | Capability-only kernel authority; attenuation     | ABI §3–5 / SEC §4    | kernel/cspace               | VV-CAP-\*                              |
| v0.4         | MemoryRegion != AddressSpace; W^X; pager faults   | ARCH §6 / ABI §13    | kernel/mm + runtime/pager   | VV-MEM-\* / VV-FAULT-01                |
| v0.5         | ExecutionContext + SchedulingContext              | ARCH §7 / ABI §14–15 | kernel/sched                | VV-SCHED-01                            |
| v0.6         | Rendezvous IPC; cap transfer; Notification        | ARCH §8 / ABI §9–12  | kernel/ipc                  | VV-IPC-\*                              |
| v0.7         | User-space drivers; IOMMU-mediated DMA            | ARCH §9              | runtime/device              | future device isolation suite          |
| v0.8         | Object identity != path; immutable system         | ARCH §10             | runtime/object              | future storage transactional suite     |
| v0.9         | Per-domain namespace; Service control/data split  | ARCH §11             | runtime/service             | future service restart/namespace suite |
| v0.10        | Identity != authority; no root bypass             | SEC                  | runtime/policy              | security negative tests                |
| v0.11        | Application != process; user-space loader         | ARCH §13             | runtime/loader              | future app launch/isolation suite      |
| v0.12        | ComputeGraph/DataObject/backend separation        | ARCH §14             | runtime/compute             | planner/backend contract tests         |
| v0.13        | Kernel caps local-only; remote authority explicit | ARCH §15             | runtime/mesh                | future partition/lease tests           |
| v0.14        | Meta-System builds immutable SystemVersion; A/B   | ARCH §16             | forge + stage0              | boot-state/recovery tests              |
| v0.15        | P0 boundary + 7 syscall ABI                       | TZ §8–10 / ABI / P0  | stage0+kernel+runtime tests | P0 acceptance                          |

# 3. P0 acceptance trace

| **Acceptance**                       | **Реализация**                       | **Verification**       |
|--------------------------------------|--------------------------------------|------------------------|
| UEFI boot                            | stage0 + kernel entry                | QEMU boot smoke        |
| 3 isolated domains                   | root-manager + AddressSpace/CSpace   | integration scenario   |
| No cross-domain memory               | MMU page tables                      | VV-MEM-01              |
| Rights enforcement                   | cap lookup/invoke                    | VV-CAP-02              |
| Cap IPC transfer                     | endpoint transfer transaction        | VV-IPC-02              |
| RPC                                  | call/reply_wait                      | VV-IPC-01              |
| User pager                           | FaultEndpoint + AS_MAP               | VV-FAULT-01            |
| Timer preemption                     | Timer/Notification/SchedulingContext | VV-SCHED-01            |
| Domain failure isolation             | fault termination/supervisor event   | fault injection        |
| No kernel panic from malformed input | syscall validators                   | fuzz + negative corpus |

# 4. Правило изменения

Изменение critical requirement должно одновременно обновлять исходную
спецификацию, RTM row, соответствующий ADR и test/verification mapping.
Код без трассируемого требования допускается только для внутреннего
implementation detail, не меняющего observable semantics.
