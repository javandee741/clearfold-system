<!-- clearfold-doc-id: README; source-language: ru; source-version: 0.1 -->
<div align="center">

# Clearfold

### Гетерогенная capability-based вычислительная система

**Computation, not Process.**  
**Система, которая создаёт систему.**

[English](README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Español](README.es.md)

</div>

> **Статус:** pre-alpha исследовательский и инженерный проект. Архитектурный baseline v0.1–v0.15 завершён; первый прототип микроядра x86-64/UEFI ещё не реализован.

## Что такое Clearfold?

Clearfold — capability-based микроядерная операционная и вычислительная система, в которой фундаментальной высокоуровневой сущностью является **Computation**, а не классический **Process**. CPU рассматривается как один из backend'ов исполнения наряду с GPU, QPU, магнитными/троичными, нейроморфными и будущими физическими вычислителями.

Система разделена на малый детерминированный Runtime и более крупную **Meta-System**, которая инвентаризирует оборудование, разрешает зависимости и политику, специализирует компоненты и строит immutable-образ Runtime для конкретной машины.

```text
Приложения
    ↓
Universal Compute API
    ↓
Compute Graph / Data Objects
    ↓
Compute Manager
    ↓
CPU · GPU · QPU · Magnetic · Bio · Remote backends
    ↓
User-space services и driver domains
    ↓
Capability microkernel
    ↓
Hardware
```

## Основные принципы

- **Kernel = mechanism, not policy.**
- Capabilities — фундаментальный механизм authority.
- Identity не является authority.
- Драйверы, файловые системы, сеть, GUI и policy находятся в user space.
- Process и File — производные/compatibility-абстракции, а не фундамент kernel.
- Kernel capabilities никогда не пересекают границу Node.
- Удалённость, latency, trust boundary и partial failure остаются явными.
- SystemVersion immutable; обновление — A/B + recovery.
- Универсальна Meta-System, а Runtime специализируется под железо.

## Архитектура

Проект зафиксирован в пятнадцати моделях v0.1–v0.15. Полный русский baseline находится в [`docs/ru/baseline/`](docs/ru/baseline/), обзор — в [`docs/ru/architecture-overview.md`](docs/ru/architecture-overview.md).

## Первый прототип P0

```text
x86-64 · UEFI · Rust + немного assembly · 1–4 CPU

UEFI → Stage-0 → Clearfold microkernel → Root Manager
                         ↓
               изолированные domains
                         ↓
            capabilities + IPC + pager
```

В P0 намеренно **нет** filesystem, network, USB, GPU, GUI, POSIX, distributed runtime и полной Meta-System. Цель — доказать работоспособность базовой модели изоляции, capabilities, memory mapping, execution contexts, scheduler, IPC, notifications, timers и user-space pager.

## Языки документации

До заморозки английской редакции **v0.15 на русском остаётся историческим нормативным baseline**. Начиная с v0.16 целевой канонический язык спецификаций — English; русский и 简体中文 поддерживаются как полные переводы. 日本語, 한국어, 繁體中文 и Español сначала получают README, сайт и наиболее важные документы.

Подробно: [`docs/TRANSLATIONS.md`](docs/TRANSLATIONS.md).

## Участие в разработке

Сейчас проект переходит от архитектуры к Prototype P0. Особенно полезны специалисты по microkernel/capability systems, Rust `no_std`, x86-64/UEFI, formal verification, user-space drivers, heterogeneous computing, документации и локализации.

Перед архитектурными изменениями прочитайте [`CONTRIBUTING.md`](CONTRIBUTING.md) и [`adr/README.md`](adr/README.md).

## Лицензия

Apache License 2.0.

## Название проекта

Название проекта — **Clearfold**, каноническое имя репозитория — **`clearfold-system`**. Apache-2.0 распространяется на исходный код и документацию; правила использования названия и визуальной идентичности описаны отдельно в [`BRANDING.md`](BRANDING.md).
