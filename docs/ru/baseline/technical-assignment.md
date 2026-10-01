<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Техническое задание на разработку операционной и вычислительной
системы**

CLEARFOLD-TZ-001 • версия 0.15

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                                               |
|------------|------------|------------|-------------------------------------------------------------|
| 0.15       | 01.10.2026 | Baseline   | Формализация требований по архитектурным моделям v0.1–v0.15 |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Общие сведения

Наименование проекта: Clearfold. Тип разработки: экспериментальная
capability-based микроядерная операционная и вычислительная система
нового поколения с Meta-System для построения специализированных
Runtime-образов и с нативной поддержкой гетерогенных вычислительных
backend'ов.

Цель разработки — создать систему, в которой фундаментальной
высокоуровневой сущностью является Computation, а не Process; CPU
является одним из вычислительных backend'ов наряду с GPU, QPU и
перспективными физическими вычислителями.

# 2. Основание и исходные архитектурные принципы

1.  Kernel = mechanism, not policy.

2.  Computation, not Process: понятие вычисления находится выше
    микроядра.

3.  Capability является единственным фундаментальным механизмом
    authority на kernel objects.

4.  Identity не является authority; фактические полномочия выражаются
    capabilities.

5.  Драйверы и системные сервисы работают в user space; kernel содержит
    только минимальную platform/architecture поддержку.

6.  Файлы, каталоги, пути, сокеты, пользователи, GPU/QPU semantics и
    Computation не являются объектами kernel.

7.  Универсальность концентрируется в Meta-System; Runtime
    специализирован под стабильное аппаратное основание машины.

8.  Kernel ABI должен быть минимальным, стабильным, детерминированным и
    пригодным для последующей формальной верификации.

9.  Local и remote ресурсы не смешиваются семантически: сеть, latency,
    trust boundary и partial failure остаются явными.

10. Обновление системы создаёт новую immutable SystemVersion и
    выполняется через A/B + recovery модель.

# 3. Назначение Clearfold

- рабочие станции и серверы с повышенными требованиями к изоляции
  компонентов;

- гетерогенные вычислительные узлы CPU/GPU/accelerators;

- исследовательская платформа для QPU, магнитных/троичных, нейроморфных
  и иных backend'ов;

- распределённые multi-node вычислительные комплексы;

- embedded/realtime профили за счёт специализации Runtime;

- исследование capability security, user-space drivers и формально
  проверяемого микроядра.

# 4. Состав системы

| **Подсистема**      | **Назначение**                                                                                                    |
|---------------------|-------------------------------------------------------------------------------------------------------------------|
| Meta-System         | Инвентаризация оборудования, dependency resolution, policy compilation, специализация, построение System Object.  |
| Stage-0             | Минимальная UEFI-загрузка, проверка System Object, Boot Manifest, передача управления microkernel.                |
| Microkernel         | Capabilities, memory mapping, execution, scheduling mechanics, IPC, notifications, timers, interrupts, MMU/IOMMU. |
| Managers            | Root Resource, Memory, Scheduler, Domain, Service, Device, Policy, Compute.                                       |
| Services            | Object/Storage, Network, GUI, Secrets, Telemetry, Compatibility и др.                                             |
| Driver Domains      | User-space hardware drivers с MMIO/IRQ/DMA capabilities.                                                          |
| Compute Runtime     | Compute Graph, DataObject, planner, backend runtimes CPU/GPU/QPU/MAG/BIO.                                         |
| Distributed Runtime | Remote capabilities, service proxies, node identity/trust, placement и replication.                               |

# 5. Функциональные требования

## 5.1 Microkernel

- ДОЛЖЕН обеспечивать изоляцию AddressSpace между domains.

- ДОЛЖЕН реализовать capability lookup и проверку прав для каждой
  операции над kernel object.

- ДОЛЖЕН обеспечивать ExecutionContext, SchedulingContext и preemption.

- ДОЛЖЕН реализовать rendezvous IPC и Notification.

- НЕ ДОЛЖЕН содержать filesystem, network stack, device protocol
  drivers, GUI, user/group policy или Computation semantics.

## 5.2 Capabilities

- Capability ДОЛЖНА быть неподлежаща подделке из user space.

- Производная capability НЕ МОЖЕТ расширять права или диапазон родителя.

- ДОЛЖНЫ поддерживаться COPY, MINT, MOVE, DELETE и REVOKE.

- Kernel capabilities действуют только внутри одного Node.

- Capability transfer через IPC ДОЛЖЕН быть атомарным.

## 5.3 Memory

- MemoryRegion и AddressSpace ДОЛЖНЫ быть разными объектами.

- Отображение возможно только при наличии соответствующей capability.

- W^X ДОЛЖЕН применяться по умолчанию.

- Page fault ДОЛЖЕН доставляться user-space FaultEndpoint/Pager.

- Политика NUMA, swap, eviction и OOM НЕ ДОЛЖНА находиться в kernel.

## 5.4 Execution & Scheduling

- Kernel scheduler ДОЛЖЕН планировать только ExecutionContext.

- CPU-time ДОЛЖЕН контролироваться SchedulingContext.

- Scheduling policy СЛЕДУЕТ держать в user-space Scheduler Manager.

- GPU/QPU/Bio execution НЕ ДОЛЖНО моделироваться как CPU
  ExecutionContext.

- Длительные kernel operations ДОЛЖНЫ быть bounded или
  incremental/preemptible.

## 5.5 IPC & Events

- Endpoint ДОЛЖЕН использовать rendezvous semantics без произвольной
  payload queue.

- Малые сообщения СЛЕДУЕТ передавать через register fast path.

- Крупные данные ДОЛЖНЫ передаваться через MemoryRegion/zero-copy схемы.

- Reply path ДОЛЖЕН поддерживать call/reply_wait.

- Notification ДОЛЖНА быть bounded event primitive.

## 5.6 Device & Driver

- Hardware-specific drivers НЕ ДОЛЖНЫ исполняться в kernel mode.

- Device access ДОЛЖЕН выражаться DeviceRegion/Interrupt/DMAContext
  capabilities.

- При наличии IOMMU DMA ДОЛЖЕН быть изолирован.

- Падение driver domain НЕ ДОЛЖНО автоматически приводить к kernel
  panic.

- Bus enumeration и device classification ДОЛЖНЫ быть user-space
  функциями.

## 5.7 Storage & Object

- Persistent data SHOULD представляться Objects с устойчивой identity.

- Path НЕ ДОЛЖЕН являться identity объекта.

- System и приложения SHOULD быть immutable/versioned.

- Persistent state changes SHOULD быть transactional.

- POSIX filesystem ДОЛЖЕН рассматриваться как compatibility service.

## 5.8 Services & Namespace

- Каждый domain ДОЛЖЕН стартовать с минимальным namespace.

- Service Manager ДОЛЖЕН находиться в control plane и не участвовать в
  обычном data path.

- Service interfaces ДОЛЖНЫ иметь независимое versioning.

- Новый domain НЕ ДОЛЖЕН автоматически наследовать полномочия parent.

- Service crash SHOULD локализоваться и допускать restart там, где
  возможно.

## 5.9 Security

- Kernel НЕ ДОЛЖЕН знать пользователей, роли и организации.

- Administrative identity НЕ ДОЛЖНА обходить capability checks.

- Policy Manager принимает решения, но SHOULD не владеть всеми
  защищаемыми ресурсами.

- Sensitive secrets SHOULD предоставляться как ограниченные операции, а
  не raw bytes.

- ДОЛЖНЫ поддерживаться revocation и trusted UI для чувствительных
  grants.

## 5.10 Applications

- Application НЕ ДОЛЖНА быть kernel process.

- Loader Service ДОЛЖЕН находиться в user space.

- Kernel ABI, Runtime ABI и Service Protocol ABI ДОЛЖНЫ быть разделены.

- Legacy POSIX/Win32/Linux semantics ДОЛЖНЫ реализовываться
  compatibility layers.

- Application update SHOULD создавать новую immutable version.

## 5.11 Heterogeneous Compute

- ComputeGraph НЕ является kernel object.

- DataObject и MemoryRegion ДОЛЖНЫ быть разными сущностями.

- Planner ДОЛЖЕН учитывать transfer cost и locality.

- Backend-specific scheduling ДОЛЖЕН оставаться вне microkernel.

- Probabilistic/approximate backends ДОЛЖНЫ возвращать quality metadata.

## 5.12 Distributed

- Каждый Node ДОЛЖЕН иметь независимый microkernel.

- Execution Domain всегда локален одному Node.

- Remote authority ДОЛЖНА быть cryptographically protected и не
  подменять local kernel capability.

- Partial failure и locality ДОЛЖНЫ быть видимы runtime.

- Strong consistency предоставляется специализированными services, а не
  kernel.

## 5.13 Boot & Construction

- Runtime ДОЛЖЕН быть результатом Meta-System construction.

- Stage-0 ДОЛЖЕН оставаться минимальным.

- System Object ДОЛЖЕН быть immutable и проверяемым.

- Обновление ДОЛЖНО создавать новую SystemVersion.

- A/B + Recovery ДОЛЖНЫ обеспечивать rollback без автоматического
  rollback User Data.

# 6. Нефункциональные требования

| **Категория**      | **Требование**                                                                                        |
|--------------------|-------------------------------------------------------------------------------------------------------|
| Безопасность       | Least authority, W^X, capability attenuation, no ambient authority, bounded privileged code.          |
| Надёжность         | Изоляция отказов user-space services/drivers; A/B rollback; recovery image.                           |
| Производительность | Минимизация IPC/context-switch overhead; zero-copy для больших данных; benchmark regression tracking. |
| Проверяемость      | Малое ядро, явные object/state transitions, property tests и последующая formal verification.         |
| Переносимость      | Architecture bindings отделены от kernel core; первая реализация x86-64, дальнейшие ARM64/RISC-V.     |
| Воспроизводимость  | System construction фиксирует inputs, hashes, policy version, compiler/toolchain provenance.          |
| Масштабируемость   | SMP, multi-node, backend-local schedulers; отсутствие обязательного глобального kernel/control path.  |

# 7. Интерфейсы и ABI

Публичный kernel ABI baseline состоит из 7 entry points: invoke, send,
recv, call, reply_wait, wait, yield. Остальные операции выражаются
методами kernel objects через invoke. Точный layout закреплён в
документе CLEARFOLD-ABI-001.

# 8. Требования к прототипу P0

P0 реализуется на x86-64/UEFI и проверяет фундамент: boot, user mode,
capability spaces, MemoryRegion/AddressSpace, ExecutionContext,
SchedulingContext, IPC, Notification, Timer, fault delivery, базовый
SMP. В P0 отсутствуют filesystem, network, USB, GPU, GUI, POSIX, swap,
NUMA и distributed runtime.

# 9. Критерии приёмки P0

11. Загрузка через UEFI и переход в microkernel.

12. Запуск user mode и не менее трёх изолированных domains.

13. Невозможность чтения/записи памяти другого domain без capability.

14. Корректный отказ при отсутствующем capability right.

15. Capability transfer через IPC с attenuation.

16. Рабочий call/reply_wait RPC.

17. Page fault обслуживается user-space Pager и выполнение
    возобновляется.

18. Timer preemption переключает ExecutionContexts и соблюдает budget.

19. Ошибка user domain не приводит к kernel panic.

20. Набор security/property tests проходит автоматически.

# 10. Этапы разработки

| **Этап** | **Результат**                      |
|----------|------------------------------------|
| M0       | UEFI -\> kernel -\> serial         |
| M1       | Memory/page tables/user mode       |
| M2       | Kernel objects + capabilities      |
| M3       | Execution/context switch/scheduler |
| M4       | IPC                                |
| M5       | Faults + Pager                     |
| M6       | Timers + preemption                |
| M7       | SMP                                |
| M8       | Capability transfer + revocation   |
| M9       | Device primitives                  |
| M10      | PCIe + user-space driver           |
| M11      | NVMe                               |

# 11. Ограничения и нецели baseline

- Не обеспечивается POSIX compatibility внутри kernel.

- Не обещается единый IR для всех физических моделей вычисления.

- Не скрывается различие local/remote ресурсов.

- Не используется AI в kernel или boot-critical path.

- Не требуется математическое доказательство всей системы до
  прототипирования; формализация применяется поэтапно к критическим
  инвариантам.

# 12. Комплект документации

| **Код**          | **Документ**                                      |
|------------------|---------------------------------------------------|
| CLEARFOLD-TZ-001   | Техническое задание                               |
| CLEARFOLD-ARCH-001 | Архитектурная спецификация                        |
| CLEARFOLD-ABI-001  | Kernel Objects, Capability и Syscall ABI          |
| CLEARFOLD-SEC-001  | Security & Threat Model                           |
| CLEARFOLD-P0-001   | План реализации Prototype P0                      |
| CLEARFOLD-VV-001   | Verification & Validation Plan                    |
| CLEARFOLD-ENG-001  | Engineering Process, ADR и управление изменениями |
