<!-- clearfold-doc-language: ru; baseline: 0.15; historical-normative: true -->

**CLEARFOLD**

**Kernel Objects, Capability Model и Syscall ABI**

CLEARFOLD-ABI-001 • версия 0.15

| **Статус**        | Базовая проектная редакция                         |
|-------------------|----------------------------------------------------|
| **Дата baseline** | 01.10.2026                                         |
| **Проект**        | Clearfold — universal heterogeneous computing system |
| **Назначение**    | Документ разработки и архитектурной фиксации       |

Ключевая формула проекта: «Computation, not Process». Универсальность
сосредоточена в Meta-System, а Runtime остаётся минимальным,
специализированным и проверяемым.

# Управление документом

| **Версия** | **Дата**   | **Статус** | **Изменение**                        |
|------------|------------|------------|--------------------------------------|
| 0.15       | 01.10.2026 | Baseline   | Первая ABI baseline для Prototype P0 |

Термины MUST/ДОЛЖЕН, SHOULD/СЛЕДУЕТ и MAY/МОЖЕТ используются в
нормативном смысле: обязательное требование, рекомендуемое требование и
допустимая возможность соответственно.

# 1. Статус ABI

Документ определяет design target ABI. До milestone M8 числа операций
могут изменяться, однако семантическая модель объектов и семь syscall
entry points считаются baseline и должны изменяться только через ADR.

# 2. Kernel object registry

| **ID** | **Object**        | **P0** | **Основные rights**                                            |
|--------|-------------------|--------|----------------------------------------------------------------|
| 1      | ResourcePool      | Да     | CREATE, QUERY; позднее SPLIT/TRANSFER                          |
| 2      | CapabilitySpace   | Да     | LOOKUP, INSERT, REMOVE, COPY/MINT/MOVE/REVOKE, QUERY           |
| 3      | MemoryRegion      | Да     | MAP_READ, MAP_WRITE, MAP_EXEC, DERIVE, TRANSFER, QUERY         |
| 4      | AddressSpace      | Да     | MAP, UNMAP, PROTECT, QUERY, CONTROL                            |
| 5      | ExecutionContext  | Да     | READ_STATE, WRITE_STATE, RESUME, SUSPEND, CONFIGURE, TERMINATE |
| 6      | SchedulingContext | Да     | QUERY, CONFIGURE, ATTACH, DETACH                               |
| 7      | Endpoint          | Да     | SEND, RECV, CALL, GRANT                                        |
| 8      | Notification      | Да     | SIGNAL, WAIT, QUERY                                            |
| 9      | Timer             | Да     | ARM, CANCEL, BIND, QUERY                                       |
| 10     | Interrupt         | Позже  | BIND, ACK, MASK, UNMASK                                        |
| 11     | DeviceRegion      | Позже  | MAP/ACCESS/BIND-specific                                       |
| 12     | DMAContext        | Позже  | MAP, UNMAP, ATTACH, DETACH, QUERY                              |

# 3. CapHandle

typedef uint32_t CapHandle;  
// P0 semantics: local slot index in current CapabilitySpace.  
// It is never a kernel pointer or globally meaningful identifier.

# 4. Capability invariants

1.  Capability нельзя изготовить из произвольного user-space integer.

2.  rights(child) ⊆ rights(parent).

3.  scope/range(child) ⊆ scope/range(parent).

4.  MINT может изменить badge только при derivation и не может расширить
    authority.

5.  MOVE выполняется атомарно.

6.  REVOKE инвалидирует descendants после завершения операции.

# 5. Capability operations

| **Операция** | **Семантика**                                                       |
|--------------|---------------------------------------------------------------------|
| COPY         | Child capability с тем же badge и теми же/меньшими rights.          |
| MINT         | Child capability с теми же/меньшими rights и новым immutable badge. |
| MOVE         | Атомарный перенос authority source -\> target.                      |
| DELETE       | Удаление только указанной capability.                               |
| REVOKE       | Удаление всех descendants capability.                               |

# 6. Syscall table

| **№** | **Entry point** | **Назначение**                                      |
|-------|-----------------|-----------------------------------------------------|
| 0     | invoke          | Object method dispatch.                             |
| 1     | send            | Rendezvous IPC send.                                |
| 2     | recv            | Receive request/message.                            |
| 3     | call            | Request + wait reply.                               |
| 4     | reply_wait      | Reply previous + atomically wait next.              |
| 5     | wait            | Notification wait with absolute monotonic deadline. |
| 6     | yield           | Voluntary CPU yield.                                |

# 7. x86-64 calling convention P0

RAX = syscall number  
RDI = arg0  
RSI = arg1  
RDX = arg2  
R10 = arg3  
R8 = arg4  
R9 = arg5  
  
RAX = Status  
additional result words: ABI-specific return registers

Архитектурные bindings для ARM64/RISC-V определяются отдельными
документами; логическая syscall semantics одинакова.

# 8. invoke contract

invoke(cap, operation, arg0, arg1, arg2, arg3)  
1. resolve cap in current CSpace  
2. validate object generation/type  
3. check semantic right  
4. validate arguments  
5. execute bounded object operation  
6. return Status/result

# 9. Fast IPC

P0 использует register fast path до 6 machine words (48 bytes на
x86-64). Payload больше fast path передаётся через явно
разделяемый/передаваемый MemoryRegion. Kernel не обязан иметь 4 KiB
inline IPC buffer в P0.

MessageInfo (64-bit target layout)  
bits 0..7 length_words  
bits 8..11 cap_count  
bits 12..15 flags  
bits 16..31 reserved/version  
bits 32..63 label

# 10. Endpoint semantics

- Endpoint не содержит произвольную payload queue.

- send без receiver блокирует sender; recv без sender блокирует
  receiver.

- Порядок среди равного effective priority — FIFO target behavior.

- call создаёт transient reply state и блокирует caller.

- reply_wait должен быть оптимизированным server loop path.

# 11. Notification

pending_bits: uint64  
signal(bits): pending_bits \|= bits  
wait(mask): matched = pending_bits & mask; consume matched; block if
zero until signal/deadline

# 12. Timer

- P0 timer one-shot.

- Время — uint64 monotonic nanoseconds.

- Periodic semantics реализуются re-arm в runtime.

# 13. AddressSpace MAP

MAP(memory_cap, region_offset, virtual_address, length, permissions)

- Проверяются alignment/range/canonical address/mapping
  conflicts/rights.

- P0 page size 4 KiB.

- WRITE + EXECUTE запрещено.

- API не обещает, что 4 KiB останется единственным page size.

# 14. ExecutionContext

- x86-64 state: RIP, RSP, RFLAGS, GPR, FS/GS as needed, extended state.

- P0 использует корректный eager save/restore FPU/SSE до оптимизаций.

- Context хранит internal references на AddressSpace, CapabilitySpace,
  SchedulingContext, FaultEndpoint.

# 15. SchedulingContext P0

priority: u8  
budget_ns: u64  
period_ns: u64  
affinity_mask: u64

# 16. Error status baseline

| **Code** | **Name**          |
|----------|-------------------|
| 0        | OK                |
| 1        | INVALID_CAP       |
| 2        | WRONG_OBJECT_TYPE |
| 3        | NO_RIGHT          |
| 4        | INVALID_ARGUMENT  |
| 5        | NO_RESOURCE       |
| 6        | WOULD_BLOCK       |
| 7        | TIMEOUT           |
| 8        | CANCELLED         |
| 9        | FAULT             |
| 10       | BUSY              |

# 17. ABI rules

7.  Никаких строковых ошибок в kernel ABI.

8.  User pointers всегда валидируются до доступа kernel.

9.  Все length/offset arithmetic должны проверяться на overflow.

10. Malformed syscall не может приводить к kernel panic.

11. ABI structures имеют version/size там, где ожидается расширение.

12. Reserved fields передаются нулями и игнорируются согласно versioning
    rules.
