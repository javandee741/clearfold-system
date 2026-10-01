# Contributing to Clearfold

Clearfold is currently moving from architecture baseline v0.15 to Prototype P0. Changes that alter architectural invariants require an ADR before implementation.

## Contribution flow

1. Check existing issues and ADRs.
2. Open an issue describing the problem and affected subsystem.
3. For architecture changes, add an ADR under `adr/`.
4. Keep changes small enough to review and test independently.
5. Add or update conformance tests for normative behavior.
6. Update English canonical documentation and mark translations stale when semantics change.

## Engineering rules

- Kernel additions require a written justification for why they cannot safely live in user space.
- Kernel interfaces must use capabilities; no ambient authority or global object lookup is accepted.
- ABI changes require explicit versioning and migration notes.
- `unsafe` Rust should be concentrated at hardware/architecture boundaries and documented with invariants.
- New runtime policy must not silently migrate into the microkernel.
- Performance optimizations must preserve security and correctness invariants.

## Commit style

Prefer scoped imperative messages, for example:

```text
kernel: add endpoint rendezvous state machine
spec: define capability MOVE atomicity
stage0: validate boot manifest checksum
```

## Languages

Normative technical changes must update English documentation once the v0.16 English baseline is frozen. Until then, the Russian v0.15 baseline is the historical source. Translation-only pull requests are welcome; see `docs/TRANSLATIONS.md`.
