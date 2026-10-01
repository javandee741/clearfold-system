<!-- clearfold-doc-id: README; source-language: en; source-version: 0.1 -->
<div align="center">

# Clearfold

### Heterogeneous capability computing system

**Computation, not Process.**  
**The system that builds the system.**

[English](README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Español](README.es.md)

![Status](https://img.shields.io/badge/status-architecture%20baseline-5b5bd6)
![Spec](https://img.shields.io/badge/spec-v0.15-12b8b0)
![Prototype](https://img.shields.io/badge/prototype-P0%20planned-f0a34a)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)

</div>

> **Project status:** pre-alpha research and engineering project. The architecture baseline is complete through v0.15; the first x86-64/UEFI microkernel prototype has not yet been implemented.

## What is Clearfold?

Clearfold is a capability-based microkernel operating and computing system designed around **Computation** rather than the traditional **Process** abstraction. CPU is treated as one execution backend among CPU, GPU, QPU, magnetic/ternary, neuromorphic and future physical computing substrates.

The project separates a small, deterministic runtime from a larger **Meta-System** that discovers hardware, resolves dependencies and policy, specializes components, and constructs an immutable runtime image for a specific machine or node.

```text
Applications
    ↓
Universal Compute API
    ↓
Compute Graph / Data Objects
    ↓
Compute Manager
    ↓
CPU · GPU · QPU · Magnetic · Bio · Remote backends
    ↓
User-space services and driver domains
    ↓
Capability microkernel
    ↓
Hardware
```

## Core principles

- **Kernel = mechanism, not policy.**
- **Capabilities are the fundamental authority mechanism.**
- **Identity is not authority.**
- **Drivers, filesystems, networking, GUI and policy live in user space.**
- **Files and processes are compatibility abstractions, not kernel foundations.**
- **Kernel capabilities never cross node boundaries.**
- **Remote latency, trust and partial failure remain explicit.**
- **System versions are immutable and updated through A/B + recovery.**
- **Universality belongs to the Meta-System; runtime images are specialized.**

## Architecture baseline

The current architecture is defined by fifteen models:

| Version | Model |
|---|---|
| v0.1 | Kernel Architecture |
| v0.2 | Kernel Object Model |
| v0.3 | Capability Model |
| v0.4 | Memory Model |
| v0.5 | Execution & Scheduling Model |
| v0.6 | IPC & Event Model |
| v0.7 | Device & Driver Model |
| v0.8 | Storage & Object Model |
| v0.9 | Service & Namespace Model |
| v0.10 | Security, Identity & Policy Model |
| v0.11 | Application & Runtime Model |
| v0.12 | Compute Graph & Heterogeneous Runtime Model |
| v0.13 | Distributed & Multi-Node Model |
| v0.14 | Boot, Meta-System & System Construction Model |
| v0.15 | Formal System Specification & Minimal Prototype Boundary |

See [`docs/en/architecture-overview.md`](docs/en/architecture-overview.md) and the Russian baseline documents in [`docs/ru/baseline/`](docs/ru/baseline/).

## Prototype P0

The first implementation target is intentionally small:

```text
x86-64 · UEFI · Rust + small assembly · 1–4 CPUs

UEFI → Stage-0 → Clearfold microkernel → Root Manager
                         ↓
             isolated user domains
                         ↓
          capabilities + IPC + pager
```

P0 explicitly excludes filesystems, networking, USB, GPU, GUI, POSIX, distributed execution and the full Meta-System. It exists to validate isolation, capabilities, memory mapping, execution contexts, scheduling, IPC, notifications, timers and user-space fault handling.

## Repository map

```text
spec/              normative architecture and ABI material
docs/              documentation and translations
stage0/            UEFI bootstrap (P0)
kernel/            microkernel core + architecture ports
runtime/           root manager, pager and later system managers
libs/              ABI, IPC and runtime libraries
tests/              conformance, isolation and performance tests
site/               GitHub Pages project site
adr/                Architecture Decision Records
.github/            workflows, issue templates and project automation
```

## Documentation languages

**Translation policy:** English is the target canonical language for new normative specifications from v0.16 onward. The historical v0.15 baseline was authored in Russian and remains authoritative until the English baseline is reviewed and frozen. Simplified Chinese is the third full-document language. Japanese, Korean, Traditional Chinese and Spanish receive project pages and high-value documents first; other translations are community-driven.

See [`docs/TRANSLATIONS.md`](docs/TRANSLATIONS.md).

## Contributing

Clearfold is currently in the architecture-to-prototype transition. Contributions are especially useful in microkernel design review, Rust `no_std`, x86-64/UEFI, capability systems, formal verification, driver isolation, heterogeneous runtimes, documentation and translation.

Read [`CONTRIBUTING.md`](CONTRIBUTING.md), [`GOVERNANCE.md`](GOVERNANCE.md), and the ADR process in [`adr/README.md`](adr/README.md) before proposing architectural changes.

## Security

Security issues should not be reported in public issues. See [`SECURITY.md`](SECURITY.md).

## License

Apache License 2.0. See [`LICENSE`](LICENSE).

## Project identity

The project name is **Clearfold** and the canonical repository slug is **`clearfold-system`**. Apache-2.0 covers source code and documentation; project naming and visual identity are addressed separately in [`BRANDING.md`](BRANDING.md).
