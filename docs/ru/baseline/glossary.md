<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Глоссарий и нормативная терминология**

CLEARFOLD-GLO-001 • версия 0.1

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
| 0.1        | 01.10.2026 | Baseline   | Единая терминология архитектурной baseline v0.1–v0.15 |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Правила терминологии

Англоязычные архитектурные имена сохраняются в API и спецификациях как
стабильные термины. Русское описание поясняет смысл, но не вводит
альтернативную сущность. Термины Process, File и Socket могут
использоваться только для compatibility layers либо сравнений, если
отдельно не оговорено иное.

# 2. Базовые термины

| **Термин**               | **Нормативный смысл**                                                                          |
|--------------------------|------------------------------------------------------------------------------------------------|
| Clearfold                  | Проект в целом: Meta-System + Runtime + Compute + distributed extensions.                      |
| Clearfold Kernel           | Минимальное capability-based microkernel.                                                      |
| Clearfold Forge            | Рабочее имя Meta-System, строящего специализированную Runtime SystemVersion.                   |
| Clearfold Runtime          | Конкретная работающая специализированная система на Node.                                      |
| Node                     | Физический/виртуальный узел с собственным microkernel instance.                                |
| Kernel Object            | Привилегированная сущность, доступная только через kernel capability.                          |
| Capability               | Неподлежащее подделке kernel authority: object + rights + badge + provenance.                  |
| CapHandle                | Локальный user-visible индекс/handle capability в CapabilitySpace; не pointer и не global ID.  |
| CapabilitySpace / CSpace | Локальный namespace kernel capabilities для domain.                                            |
| ResourcePool             | Явный ресурсный источник для создания kernel objects/MemoryRegions.                            |
| MemoryRegion             | Объект классической адресуемой памяти/backing; не virtual address.                             |
| AddressSpace             | Virtual address space CPU domain-а.                                                            |
| ExecutionContext         | CPU architectural execution state; фундаментальное исполняемое ядром состояние.                |
| SchedulingContext        | CPU budget/period/priority/affinity, отделённые от ExecutionContext.                           |
| Execution Domain         | User-space агрегат AddressSpace + CSpace + contexts + namespace; не kernel object.             |
| Endpoint                 | Синхронный rendezvous IPC kernel object.                                                       |
| Notification             | Bounded asynchronous event-bit kernel object.                                                  |
| Application              | Установленная immutable логическая программная сущность/Application Object.                    |
| Application Instance     | Конкретный запуск приложения; может включать несколько local domains.                          |
| Service                  | User-space функциональный компонент, экспортирующий versioned interface через capabilities.    |
| Service Session          | Ограниченный per-client логический доступ к service.                                           |
| Namespace                | User-space отображение имён/ролей на capabilities/object/service references; не kernel CSpace. |
| Object                   | Persistent/storage logical entity с ObjectID, content/version/metadata.                        |
| ObjectID                 | Стабильная identity storage object, независимая от path/location.                              |
| ContentID                | Hash конкретного immutable content.                                                            |
| DataObject               | Логические данные Compute layer; могут иметь несколько location replicas; не MemoryRegion.     |
| Computation              | Высокоуровневая вычислительная работа, не равная process/thread.                               |
| ComputeGraph             | Graph вычислительных nodes, data dependencies и contracts.                                     |
| ComputeNode              | Достаточно крупная вычислительная операция/stage с допустимыми implementations.                |
| Backend                  | Среда исполнения computation: CPU/GPU/QPU/constraint/magnetic/bio/etc.                         |
| Compute Manager          | User-space planner/orchestrator, выбирающий backend и data movement.                           |
| System Object            | Immutable результат Meta-System construction.                                                  |
| SystemVersion            | Идентифицируемая immutable версия Runtime system composition.                                  |
| Boot Manifest            | Нормализованная бинарная platform/boot структура, передаваемая Stage-0 kernel/root manager.    |
| Stage-0                  | Минимальный boot loader; проверяет и загружает уже построенную систему.                        |
| Remote Capability        | User-space cryptographic remote authority; никогда не является kernel capability другого Node. |
| Policy                   | Правила, преобразующие identity/context/request в решение о выдаче/ограничении authority.      |
| Identity                 | Кто/что субъект; сама по себе не предоставляет authority.                                      |
| Authority                | Фактическая возможность операции, в Clearfold выражаемая capability/session authority.           |

# 3. Термины, не являющиеся фундаментальными

| **Термин**          | **Статус**                                                                               |
|---------------------|------------------------------------------------------------------------------------------|
| Process             | Compatibility/runtime abstraction; kernel его не знает.                                  |
| Thread              | Language/runtime abstraction; kernel фундаментально оперирует ExecutionContext.          |
| File/Directory/Path | Storage/namespace abstractions; не kernel objects.                                       |
| Socket              | Network service abstraction; не kernel object.                                           |
| User/Group/Admin    | Identity/policy concepts; kernel checks не основаны на них.                              |
| Driver              | User-space domain/service; не kernel-resident subsystem.                                 |
| GPU/QPU object      | Backend/runtime semantics; kernel видит только low-level device/memory/event mechanisms. |

# 4. Нормативные выражения

| **Форма**            | **Смысл**                                                 |
|----------------------|-----------------------------------------------------------|
| ДОЛЖЕН / MUST        | Обязательное требование baseline.                         |
| НЕ ДОЛЖЕН / MUST NOT | Запрещённое baseline поведение.                           |
| СЛЕДУЕТ / SHOULD     | Предпочтительное решение; отклонение требует обоснования. |
| МОЖЕТ / MAY          | Допустимая опция.                                         |
