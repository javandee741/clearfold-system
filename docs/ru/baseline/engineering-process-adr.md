<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Engineering Process, ADR и управление изменениями**

CLEARFOLD-ENG-001 • версия 0.1

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                                         |
|------------|------------|------------|-------------------------------------------------------|
| 0.1        | 01.10.2026 | Baseline   | Инженерный процесс для архитектурно чувствительной ОС |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Принцип управления архитектурой

Clearfold должна противостоять постепенному превращению микроядра в
монолит. Любое расширение privileged surface рассматривается как
архитектурное исключение и документируется через Architecture Decision
Record (ADR).

# 2. Документные статусы

| **Статус**   | **Смысл**                                                      |
|--------------|----------------------------------------------------------------|
| DRAFT        | Обсуждается; реализация не обязана соответствовать.            |
| BASELINE     | Принято для текущего milestone.                                |
| EXPERIMENTAL | Разрешено для prototype/measurement без обещания стабильности. |
| STABLE       | Совместимость должна сохраняться или мигрироваться.            |
| DEPRECATED   | Поддерживается временно; указан replacement/removal plan.      |

# 3. ADR обязателен, если

- добавляется syscall entry point;

- добавляется kernel object type;

- добавляется новое semantic capability right;

- переносится policy в kernel;

- появляется новый privileged parser/driver;

- меняется capability revocation/transfer semantics;

- меняется stable ABI;

- нарушается один из архитектурных invariants v0.1–v0.15.

# 4. ADR template

ADR-XXXX: \<Название\>  
Status: Proposed / Accepted / Rejected / Superseded  
Date:  
Context:  
Decision:  
Alternatives considered:  
Security impact:  
Performance impact:  
Formal-verification impact:  
Migration/compatibility:  
Consequences:  
Rollback plan:  
Related specifications/tests:

# 5. Kernel feature admission checklist

1.  Нельзя ли безопасно реализовать функцию в user space?

2.  Требуется ли privileged instruction/state для корректности?

3.  Какова новая authority surface?

4.  Как failure влияет на другие domains?

5.  Можно ли ограничить операцию capabilities?

6.  Является ли операция bounded/preemptible?

7.  Как её fuzz/test/formally model?

8.  Нужен ли новый syscall или достаточно invoke(object, op)?

# 6. Code review checklist

- Нет unchecked arithmetic на offsets/sizes.

- Нет dereference user pointer до validate/copy boundary.

- Capability rights проверяются до object mutation.

- Lock ordering/interrupt state задокументированы.

- Kernel allocation bounded/accounted.

- Error path не оставляет partial capability transfer/mapping.

- unsafe region минимален и имеет safety comment.

- Тест добавлен для исправленного invariant violation.

# 7. Naming baseline

| **Имя**         | **Значение**                               |
|-----------------|--------------------------------------------|
| Clearfold         | Весь проект/система.                       |
| Clearfold Kernel  | Microkernel.                               |
| Clearfold Forge   | Рабочее имя Meta-System/build subsystem.   |
| Clearfold Runtime | Специализированная работающая ОС.          |
| Clearfold Compute | Heterogeneous Compute layer.               |
| Clearfold Mesh    | Multi-node/distributed layer; рабочее имя. |

# 8. Release policy для prototype

- 0.x допускает ABI break с ADR и migration note.

- Каждый milestone содержит exact commit/toolchain/build provenance.

- System Object/boot artifact имеет hash и build manifest.

- Prototype benchmarks сохраняются как исторические артефакты.

# 9. Структура документации в репозитории

/spec  
00-principles.md  
kernel-objects.md  
capability-model.md  
syscall-abi.md  
memory-model.md  
execution-scheduling.md  
ipc-events.md  
boot-manifest.md  
security-invariants.md  
/adr  
/tests/spec-matrix  
/docs/architecture
