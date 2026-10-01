# Clearfold Governance

Clearfold starts as a maintainer-led research and engineering project. The goal is to keep architectural decisions explicit, reviewable and reproducible while the first prototype is built.

## Decision classes

- **Implementation decision:** local code choice that does not change a published invariant or ABI.
- **Architecture decision:** changes object semantics, authority, ABI, service boundaries, boot model or system construction. Requires ADR.
- **Normative specification change:** changes MUST/SHOULD/MAY behavior. Requires review, traceability update and version bump.

## Current maintainer

Repository owner: `@javandee741`.

## Future evolution

When multiple regular maintainers exist, governance should move to a small technical steering group with subsystem ownership and documented merge authority. No governance mechanism may override the security invariants merely for convenience.
