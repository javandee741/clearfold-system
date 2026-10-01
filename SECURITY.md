# Security Policy

Clearfold is pre-alpha and must not yet be used to protect production systems or sensitive workloads.

## Reporting a vulnerability

Do **not** publish exploit details in a public GitHub issue. Use GitHub's private vulnerability reporting feature when it is enabled for the repository. If private reporting is unavailable, open a minimal public issue asking the maintainer for a private contact channel without including technical exploit details.

## Security-sensitive areas

Highest-priority review areas include:

- capability lookup, attenuation, transfer and revocation;
- address-space isolation and page-table manipulation;
- IPC capability transfer atomicity;
- syscall argument validation;
- boot manifest parsing and trust chain;
- IOMMU/DMA isolation when device support is introduced;
- remote capability token validation in later multi-node prototypes.

## Supported versions

No stable security-supported release exists yet. Security support begins only after the project publishes an explicit supported-release policy.
