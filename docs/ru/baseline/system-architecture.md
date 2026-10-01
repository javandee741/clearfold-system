<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Архитектурная спецификация системы**

CLEARFOLD-ARCH-001 • версия 0.15

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                         |
|------------|------------|------------|---------------------------------------|
| 0.15       | 01.10.2026 | Baseline   | Свод архитектурных моделей v0.1–v0.14 |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Архитектурная формула

Meta-System -\> System Object -\> Stage-0 -\> Microkernel -\>
Managers/Services -\> Applications  
-\> Compute Manager -\> CPU/GPU/QPU/MAG/BIO

Clearfold рассматривает операционную систему как механизм управления
authority, данными и вычислениями над гетерогенным вычислительным
комплексом. Микроядро не является носителем высокоуровневой системной
семантики.

# 2. Слои

| **Слой**         | **Сущности**                                                                                          | **Граница ответственности**            |
|------------------|-------------------------------------------------------------------------------------------------------|----------------------------------------|
| L0 Hardware      | CPU, RAM, MMU/IOMMU, IRQ, timers, buses                                                               | Физическое исполнение.                 |
| L1 Microkernel   | Capabilities, MemoryRegion, AddressSpace, ExecutionContext, SchedulingContext, Endpoint, Notification | Изоляция и privileged mechanisms.      |
| L2 Core Managers | Root/Memory/Scheduler/Domain/Service/Device/Policy                                                    | Политика и orchestration.              |
| L3 Services      | Storage, Network, GUI, Secrets, Telemetry                                                             | Системная функциональность user space. |
| L4 Compute       | ComputeGraph, DataObject, planner, backends                                                           | Гетерогенное вычисление.               |
| L5 Applications  | Application Object, Runtime, sessions                                                                 | Пользовательская логика.               |
| L6 Distributed   | Remote proxy/capabilities, node services                                                              | Multi-node расширение.                 |
| Meta             | Meta-System, construction, A/B/recovery                                                               | Построение и обновление Runtime.       |

# 3. v0.1 Kernel Architecture

- Минимальное микроядро; mechanism not policy.

- Kernel не знает Computation.

- Drivers и системные сервисы находятся вне kernel.

- Root Resource Manager получает первоначальные capabilities из Boot
  Manifest.

# 4. v0.2 Kernel Object Model

| **Object**        | **Назначение**                                                 |
|-------------------|----------------------------------------------------------------|
| ResourcePool      | Явный источник ресурсов для создания kernel objects.           |
| CapabilitySpace   | Локальный namespace неподлежающих подделке capabilities.       |
| MemoryRegion      | Физически обеспеченная область классической адресуемой памяти. |
| AddressSpace      | CPU virtual address space.                                     |
| ExecutionContext  | Архитектурное CPU execution state.                             |
| SchedulingContext | CPU budget/period/priority/affinity.                           |
| Endpoint          | Синхронный IPC rendezvous.                                     |
| Notification      | Bounded asynchronous event bits.                               |
| Timer             | Monotonic deadline source.                                     |
| Interrupt         | Capability-controlled IRQ representation.                      |
| DeviceRegion      | MMIO/I/O hardware resources without device semantics.          |
| DMAContext        | IOMMU-mediated DMA authority.                                  |

# 5. v0.3 Capability Model

Capability = защищённая kernel-ссылка на объект + semantic rights +
immutable badge + provenance. User space видит только локальный
CapHandle. Производные capabilities только ослабляют authority.
COPY/MINT/MOVE/DELETE/REVOKE формируют capability derivation tree.

# 6. v0.4 Memory Model

- MemoryRegion отделён от AddressSpace.

- Pages — аппаратный механизм, не публичная фундаментальная abstraction.

- Page faults маршрутизируются Pager/Memory Manager через FaultEndpoint.

- Shared memory = один MemoryRegion с capabilities у нескольких domains.

- DMA доступ проходит через DMAContext/IOMMU.

- QPU/Bio state не притворяется MemoryRegion.

# 7. v0.5 Execution & Scheduling

- Kernel scheduler видит только ExecutionContexts и SchedulingContexts.

- Scheduling policy находится в user space.

- Compute Manager не является CPU scheduler.

- Scheduling-context donation предусмотрена для RPC/accounting, но
  вводится поэтапно.

- Tickless timers и короткие bounded kernel paths являются целевыми
  свойствами.

# 8. v0.6 IPC & Event Model

- Endpoint — rendezvous без payload queue.

- call/reply_wait — основной RPC fast path.

- Large data — MemoryRegion, а не копирование через kernel.

- Notification — 64-bit bounded event primitive.

- Fault delivery использует тот же IPC/event фундамент.

# 9. v0.7 Device & Driver

- Driver — изолированный user-space domain.

- Bus/Driver/Device Managers работают в control plane.

- Приложение получает Device Service Session, а не raw MMIO.

- Crash driver-а локализуется; lifecycle включает
  probe/init/quiesce/reset/recover/shutdown.

# 10. v0.8 Storage & Object

- ObjectID отделён от path/name.

- Namespace — отображение имён/ссылок на authority и objects.

- System/App trees immutable/versioned; mutable state transactional.

- Content-addressed storage используется там, где это уместно.

- POSIX filesystem — personality/compatibility service.

# 11. v0.9 Service & Namespace

- Service = user-space domain/group + interface + capabilities.

- Каждый domain получает индивидуальный initial namespace.

- Control plane выдаёт capability, data plane затем работает напрямую.

- Versioned interface IDs позволяют параллельно поддерживать несколько
  версий.

# 12. v0.10 Security, Identity & Policy

- Identity -\> Policy -\> Capability; identity сама по себе authority не
  даёт.

- Нет абсолютного UID 0/root bypass.

- Trusted UI используется для чувствительных grants.

- Policy Manager принимает решения, но не должен владеть всем ресурсным
  миром.

- Secure/measured boot дополняет, но не заменяет capability sandbox.

# 13. v0.11 Application & Runtime

- Application Object immutable и содержит
  manifest/code/resources/dependencies.

- Application Instance создаёт один или несколько local Execution
  Domains.

- Native Runtime API строится вокруг Tasks, Channels, Objects, Services
  и Compute.

- POSIX/Linux/Win32 реализуются personalities/VM fallback.

- Portable IR может AOT-компилироваться Meta-System под конкретное
  hardware.

# 14. v0.12 Compute Graph

- ComputeGraph описывает semantics и dependencies, не ISA.

- DataObject имеет logical identity и location/replica state.

- Planner учитывает compute, transfer, queue, synchronization, energy,
  reliability и policy constraints.

- Backend runtimes различны по semantics; не создаётся фиктивная
  универсальная ISA.

- Pure deterministic nodes пригодны для retry/cache/migration.

# 15. v0.13 Distributed & Multi-Node

- Каждый Node автономен и имеет собственное microkernel.

- Kernel capability не пересекает сеть.

- Remote capability — user-space cryptographic authority + local proxy.

- Leases/expiry помогают distributed revocation.

- Execution Domain всегда локален; cluster planner занимается placement,
  а local kernel scheduler — CPU ticks.

# 16. v0.14 Boot & Meta-System

- Meta-System превращает hardware inventory + policy + repository в
  System Graph/System Object.

- Stage-0 только проверяет/загружает существующую SystemVersion.

- Boot Manifest нормализует platform facts и initial authority.

- Обновления A/B; Recovery — отдельная immutable версия.

- User Data отделены от Runtime и переживают реконструкцию системы.

# 17. Основные потоки

## 17.1 Запуск приложения

Application Object -\> Loader -\> Policy -\> Domain Manager -\>
AddressSpace/CSpace/SC -\> Initial Namespace -\> RESUME

## 17.2 Чтение объекта

App -\> ObjectCapability -\> Object Service -\> Cache/Pager -\> Block
Service -\> Driver -\> Device

## 17.3 Compute placement

Graph -\> Security filter -\> candidate backends -\> cost estimation -\>
data movement plan -\> reservation -\> ExecutionPlan

## 17.4 Remote backend

App -\> local ComputeSession -\> Remote Proxy -\> secure transport -\>
remote service -\> local capabilities -\> backend

# 18. Архитектурные запреты

- Не вводить kernel feature без доказательства невозможности безопасной
  реализации вне privileged mode.

- Не добавлять process/file/socket/user как фундаментальные kernel
  abstractions.

- Не помещать AI/ML decision making в microkernel или Stage-0.

- Не скрывать remote latency/failure semantics.

- Не превращать Service Manager или Policy Manager в обязательный
  data-path bottleneck.
