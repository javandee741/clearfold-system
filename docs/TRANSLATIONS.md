# Documentation translation policy

## Language tiers

| Tier | Locale | Scope |
|---|---|---|
| Canonical target | `en` | Normative specifications from v0.16 onward |
| Tier 1 | `ru` | Full documentation; historical v0.15 source baseline |
| Tier 1 | `zh-CN` | Full documentation translation |
| Tier 2 | `zh-TW` | Landing page, README, core architecture/security docs |
| Tier 2 | `ja` | Landing page, README, core architecture/runtime docs |
| Tier 2 | `ko` | Landing page, README, core architecture/compute docs |
| Tier 2 | `es` | Landing page, README, architecture and contribution docs |
| Tier 3 | `pt-BR`, `de`, `fr` | Community-driven / demand-driven |

## Why this order?

English is the shared language of international systems research and open-source development. Russian preserves the original architectural work. Simplified Chinese deserves full coverage because of the size of the systems, hardware, embedded and accelerator engineering community. Japanese, Korean and Traditional Chinese are particularly relevant to hardware, semiconductor and systems communities. Spanish provides broad geographic reach. For Indian technical communities, English is a better first investment than a separate Hindi edition.

## Source-of-truth rule

Each translated document carries:

```text
doc-id
source-language
source-version
translation-version
translation-status
```

When normative content changes, translations are marked `STALE` until synchronized. A translation may improve wording, but it must not introduce new normative requirements.

## Directory layout

```text
docs/
  en/
  ru/
  zh-CN/
  zh-TW/
  ja/
  ko/
  es/
```

Generated PDF/DOCX packages should be attached to GitHub Releases; Markdown remains the reviewable source inside Git.
