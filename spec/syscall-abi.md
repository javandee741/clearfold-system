# Syscall ABI — v0.15 baseline

The design target contains seven kernel entry points:

```text
0 invoke
1 send
2 recv
3 call
4 reply_wait
5 wait
6 yield
```

Object-specific control operations are dispatched through `invoke`. Large application/service APIs are not syscalls.

For x86-64 P0, the tentative syscall register convention is documented in the formal ABI document and must be frozen before P0/M2 is considered complete.
