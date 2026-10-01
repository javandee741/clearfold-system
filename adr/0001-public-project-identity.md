# ADR-0001: Public project identity

- **Status:** Accepted
- **Date:** 2026-10-01

## Context

The project needs a stable public identity before repository publication, package namespaces, documentation links and releases begin. An earlier internal codename was retired before implementation so the public name could be reconsidered before package namespaces, releases and external adoption made renaming expensive.

A preliminary comparison of candidate names included software/OS projects, package ecosystems, companies and public trademark records. **Plainweave** was rejected after an exact-name developer-tool/package collision was found. **Clearfold** was selected because no direct exact-name operating-system, kernel or runtime project was found in the preliminary search, while known exact-name uses were either outside software or did not present the same degree of ecosystem collision. Professional trademark clearance remains a separate future step.

## Decision

Use **Clearfold** as the project name. Use the formal descriptor **Clearfold — Heterogeneous Capability Computing System** and repository slug **`clearfold-system`**.

Do not make `OS` part of the primary brand: the project includes the microkernel, runtime, heterogeneous compute model, distributed layer and Meta-System.

## Consequences

- Public documentation and repositories use Clearfold.
- Package/repository namespaces should prefer `clearfold-*` plus a subsystem noun.
- The Apache-2.0 license does not grant rights to the Clearfold name or visual identity; see `BRANDING.md`.
