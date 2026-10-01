<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Security Model и Threat Model**

CLEARFOLD-SEC-001 • версия 0.10

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                                          |
|------------|------------|------------|--------------------------------------------------------|
| 0.10       | 01.10.2026 | Baseline   | Capability security, identity/policy и основные угрозы |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Цели безопасности

- Изоляция памяти и execution domains.

- Отсутствие ambient authority.

- Неподлежащее подделке и ослабляемое делегирование authority.

- Локализация driver/service compromise.

- Контролируемый bootstrap, update и recovery.

- Разделение identity, policy decision и actual authority.

# 2. Trust boundaries

| **Граница**        | **Доверие**                                                                    |
|--------------------|--------------------------------------------------------------------------------|
| Microkernel        | Минимальный постоянный TCB.                                                    |
| Stage-0            | Boot TCB: verification/load only.                                              |
| Root/Core Managers | Привилегированные user-space компоненты; capabilities ограничивают scope.      |
| Drivers            | По умолчанию потенциально ошибочны/скомпрометированы; user-space isolation.    |
| Applications       | Недоверенные по умолчанию.                                                     |
| Meta-System        | Мощный build/recovery компонент, желательно не постоянный Runtime TCB.         |
| Remote nodes       | Отдельные trust domains; require authentication/policy/attestation where used. |

# 3. Security decisions

Identity + Context + Policy -\> Decision -\> Capability/Session -\>
Actual Authority

Kernel не знает user, role, organization или application identity. Он
проверяет только capability/object/right. Policy Manager не должен
становиться абсолютным владельцем всех ресурсов.

# 4. Основные угрозы и меры

| **Угроза**                           | **Контроль**                                                             |
|--------------------------------------|--------------------------------------------------------------------------|
| Подделка handle                      | CSpace lookup; handle не является pointer/token authority сам по себе.   |
| Privilege escalation                 | Rights attenuation; semantic rights; no UID0 bypass.                     |
| Driver memory corruption             | Driver domain + MMU; IOMMU/DMAContext при доступности.                   |
| Confused deputy                      | Explicit delegated capabilities вместо глобальных path/identity claims.  |
| IPC DoS                              | Rendezvous; no unbounded payload queues; resource budgets.               |
| Executable injection                 | W^X; immutable/signed application/system objects; policy.                |
| Supply-chain compromise              | Hashes/signatures/provenance; candidate build; A/B/recovery.             |
| Stale authority                      | REVOKE for local caps; leases/expiry for remote authority.               |
| Compromised remote node              | Node identity/trust, scope, no network-neighbor ambient trust.           |
| Policy service crash                 | Existing capabilities keep semantics; new grants fail closed.            |
| Malicious app asks for all resources | Manifest is request only; Policy + Trusted UI + user-driven selection.   |
| Secret theft                         | Secrets Service exposes operations instead of raw secret where possible. |

# 5. Administrative model

Clearfold не вводит абсолютного root-пользователя, который обходил бы
kernel checks. Административные операции получают ограниченные и
предпочтительно временные capabilities после policy/reauthentication.
Recovery/break-glass authority является отдельным режимом с усиленной
защитой.

# 6. Secure boot chain

Firmware -\> Stage-0 -\> Microkernel -\> System Object -\> Critical
bootstrap/services

- System Object immutable и проверяется по hash/signature.

- Measured boot может фиксировать measurements в TPM/HSM-подобном
  сервисе.

- SecurityEpoch предотвращает запрещённый downgrade; recovery owner path
  сохраняется.

# 7. Threat assumptions

- Ошибка user-space приложения ожидаема и не должна влиять на kernel
  integrity.

- Ошибка driver-а ожидаема; устройство может зависнуть/испортить
  собственное состояние, но не должно получать arbitrary kernel memory
  authority.

- Физический доступ к машине рассматривается отдельной политикой и не
  полностью решается software архитектурой.

- Компрометация firmware/hardware root of trust находится за пределами
  гарантий pure software kernel.

# 8. Формализуемые security properties

1.  Capability Safety: отсутствие capability с правом R делает operation
    requiring R недостижимой из user space.

2.  Rights Attenuation: derived authority не расширяет parent authority.

3.  Address-Space Isolation: mapping возникает только из разрешённого
    MemoryRegion.

4.  IPC Transfer Atomicity: message/authority transfer не имеет частично
    committed состояния.

5.  Revocation Correctness: после завершённого revoke descendants не
    разрешаются.

6.  Budget Enforcement: context не превышает выделенный CPU budget сверх
    ограниченного accounting overhead.

# 9. Security review gates

- Любой новый kernel object/right требует threat review и ADR.

- Любой новый privileged service должен документировать authority и
  compromise blast radius.

- Raw device assignment требует отдельного policy path.

- Remote delegation по умолчанию non-redelegable.

- Новые parser-ы в Stage-0/kernel запрещаются без строгой необходимости.
