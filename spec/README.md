# Clearfold specification tree

`spec/` contains normative interfaces and machine-checkable or implementation-oriented specifications. Narrative architecture belongs under `docs/`; decisions belong under `adr/`.

Initial v0.15 specification targets:

```text
spec/
  kernel-objects.md
  capability-model.md
  syscall-abi.md
  ipc-abi.md
  boot-manifest.md
  invariants.md
```

These files are intentionally concise. Longer rationale belongs in architecture documentation.
