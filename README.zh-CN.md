<!-- clearfold-doc-id: README; source-language: zh-CN; source-version: 0.1 -->
<div align="center">

# Clearfold

### 异构能力（Capability）计算系统

**Computation, not Process.**  
**构建系统的系统。**

[English](README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Español](README.es.md)

</div>

> **项目状态：** pre-alpha 研究与工程项目。v0.1–v0.15 架构基线已经完成；首个 x86-64/UEFI 微内核原型尚未实现。

## Clearfold 是什么？

Clearfold 是一个基于 capability 的微内核操作与计算系统。它把 **Computation（计算）** 而不是传统的 **Process（进程）** 作为高层核心抽象。CPU 只是执行后端之一，与 GPU、QPU、磁性/三进制、神经形态以及未来的物理计算后端并列。

系统将小型、确定性的 Runtime 与更大的 **Meta-System** 分离。Meta-System 负责硬件发现、依赖与策略解析、组件特化，并为具体机器或节点构建不可变 Runtime 镜像。

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
User-space services / driver domains
    ↓
Capability microkernel
    ↓
Hardware
```

## 核心原则

- **Kernel = mechanism, not policy.**
- Capability 是内核对象权限的基础机制。
- Identity 不等于 Authority。
- 驱动、文件系统、网络、GUI 和策略位于用户态。
- Process 和 File 是兼容层抽象，而不是内核基础。
- 内核 capability 永远不会直接跨越 Node 边界。
- 远程延迟、信任边界和部分故障必须显式可见。
- SystemVersion 不可变，更新采用 A/B + recovery。
- 通用性属于 Meta-System；Runtime 针对实际硬件特化。

## P0 原型

首个实现目标为 x86-64 + UEFI，使用 Rust 与少量汇编，实现 capabilities、地址空间、ExecutionContext、基础调度、IPC、Notification、Timer 和用户态 pager。文件系统、网络、USB、GPU、GUI、POSIX 与分布式运行时不属于 P0。

## 文档语言

从 v0.16 开始，新的规范以 English 为目标权威文本；Русский 和简体中文作为完整翻译维护。v0.15 最初以俄文编写，在英文版完成审阅和冻结之前，俄文仍是该历史版本的权威基线。日本語、한국어、繁體中文和 Español 优先提供项目主页、README 与高价值文档。

详见 [`docs/TRANSLATIONS.md`](docs/TRANSLATIONS.md)。

## 许可证

Apache License 2.0。

## 项目标识

项目名称为 **Clearfold**，规范仓库名为 **`clearfold-system`**。Apache-2.0 适用于源代码和文档；项目名称与视觉标识的使用规则单独见 [`BRANDING.md`](BRANDING.md)。
