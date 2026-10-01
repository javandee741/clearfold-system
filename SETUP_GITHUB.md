# Publishing this starter repository

Recommended public repository: `javandee741/clearfold-system`.

## 1. Create an empty repository

On GitHub create **clearfold-system** under `javandee741`. Do not initialize it with a README, license or `.gitignore`; those already exist here.

Recommended settings:

- Visibility: Public
- Description: `Heterogeneous capability computing system — Computation, not Process.`
- Topics: `operating-system`, `microkernel`, `capability-security`, `rust`, `heterogeneous-computing`, `uefi`, `systems-programming`, `qpu`, `distributed-systems`

## 2. Push the starter tree

```bash
git init
git branch -M main
git add .
git commit -m "chore: bootstrap Clearfold project repository"
git remote add origin git@github.com:javandee741/clearfold-system.git
git push -u origin main
```

## 3. Enable GitHub Pages

Repository **Settings → Pages → Build and deployment → Source: GitHub Actions**.

The included `.github/workflows/pages.yml` publishes the static `site/` directory.

Expected URL:

```text
https://javandee741.github.io/clearfold-system/
```

## 4. Repository social preview

Create a dedicated 1280×640 social card from the Clearfold visual identity when branding is finalized.

## 5. First release

Create release/tag `v0.15-architecture` after the rebranded PDF/DOCX/PPTX artifacts are regenerated and reviewed.

## 6. Recommended repository features

Enable:

- Issues
- Discussions
- Private vulnerability reporting
- Branch protection for `main` once P0 development begins
- Require pull request before merge
- Require Actions checks after CI becomes implementation-critical
