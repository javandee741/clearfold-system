<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Verification & Validation Plan**

CLEARFOLD-VV-001 • версия 0.1

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                                                   |
|------------|------------|------------|-----------------------------------------------------------------|
| 0.1        | 01.10.2026 | Baseline   | План тестирования, инвариантов и будущей формальной верификации |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Стратегия

Clearfold использует ступенчатую проверку: unit tests -\> property tests
-\> fuzzing -\> integration scenarios -\> SMP stress -\> performance
regression -\> формальная модель отдельных критических инвариантов.
Формальная верификация не блокирует раннее прототипирование, но
архитектура должна сохранять проверяемость.

# 2. Test classes

| **Класс**         | **Примеры**                                                                |
|-------------------|----------------------------------------------------------------------------|
| Unit              | CSpace slot operations, bit rights, range checks, scheduler accounting.    |
| Property          | attenuation, move uniqueness, revoke reachability, address non-overlap.    |
| Fuzz              | syscall args, Boot Manifest, IPC transfer descriptors, mapping arithmetic. |
| Integration       | echo RPC, Pager fault flow, capability transfer.                           |
| Stress            | parallel IPC/revoke/use, scheduler queues, TLB shootdown.                  |
| Fault injection   | domain crash, malformed state, timer storms, interrupted IPC.              |
| Security negative | guessed handles, rights escalation, RWX map, arbitrary physical mapping.   |
| Performance       | syscall/IPC/context switch/page fault regressions.                         |

# 3. Acceptance test matrix

| **ID**      | **Требование**               | **Ожидаемый результат**                                 |
|-------------|------------------------------|---------------------------------------------------------|
| VV-CAP-01   | Forged handle                | INVALID_CAP; kernel remains healthy.                    |
| VV-CAP-02   | Child asks extra rights      | Creation denied.                                        |
| VV-CAP-03   | MOVE                         | Source invalid, target valid atomically.                |
| VV-MEM-01   | Cross-domain access          | Fault; no data disclosure.                              |
| VV-MEM-02   | RWX mapping                  | Denied.                                                 |
| VV-IPC-01   | call/reply_wait              | Correct reply and caller wakeup.                        |
| VV-IPC-02   | Capability transfer          | Transferred cap is attenuated as requested.             |
| VV-FAULT-01 | Unmapped access              | Fault message -\> pager -\> map -\> resume.             |
| VV-SCHED-01 | Budget exhaustion            | Context stops until replenishment.                      |
| VV-SMP-01   | Unmap on active multi-CPU AS | No stale accessible mapping after shootdown completion. |

# 4. Fuzzing targets

- Boot Manifest binary parser.

- invoke operation dispatch.

- Capability transfer descriptors.

- AddressSpace MAP/UNMAP ranges and overflows.

- ExecutionContext state write validation.

- IPC MessageInfo parsing.

# 5. Formal model backlog

| **Model**                    | **Цель**                                                                     |
|------------------------------|------------------------------------------------------------------------------|
| Capability algebra           | Non-forgeability assumption + attenuation + derivation/revocation semantics. |
| Memory mapping state machine | Only authorized MemoryRegion pages become user mappings.                     |
| IPC transaction              | Atomic message + cap move/copy semantics.                                    |
| Scheduler budget             | Bound consumption per period.                                                |
| Boot state A/B               | Atomic candidate/fallback transitions.                                       |

# 6. Definition of a kernel panic bug

- Любой user-controlled input, способный вызвать panic, является
  критическим дефектом.

- Use-after-free/stale capability resolution — критический дефект.

- Нарушение page-table isolation или IOMMU isolation — критический
  дефект.

- Двойная authority после exclusive MOVE — критический дефект.

# 7. CI gates

1.  Build debug/release для x86-64 target.

2.  Unit/property tests host-side, где возможно.

3.  QEMU boot smoke test.

4.  Automated user-mode isolation tests.

5.  Fuzz corpus regression.

6.  IPC/context-switch benchmark history; значимое ухудшение требует
    объяснения/ADR.

7.  Static checks на unsafe usage и forbidden dependencies kernel core.
