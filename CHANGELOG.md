# Changelog

## Ready for release

### Added
- New `analyze` command for deep transaction inspection without signing.
- Protocol Extension system for specialized analysis (Light Protocol support).
- Privacy impact classification (Public vs Compressed/Hybrid/Confidential).

## [0.1.1]

- Docs: fix README installation commands and CLI examples (stdin/pipeline, jq snippets).
- Release: bump core + CLI crates to `0.1.1` (no functional changes).

## [0.1.0]

### Added
- Core support for legacy and v0 transaction parsing/serialization and analysis models.
- Optional Address Lookup Table (ALT/LUT) resolution for v0 analysis.
- CLI sign‑tx summary output (fees/transfers), `--max-fee`, `--summary-json`, and `--tables`.
- Pipeline‑friendly behavior with stdout/stderr separation and `--yes` flow.
- Environment variable config for keypair, output format, JSON, force, yes, and max fee.
- Documentation/examples updated to reflect offline signer workflows and CI usage.
- Presenter layer used for consistent CLI output formatting.
- CLI error reporting and exit‑code mapping aligned with core error types.
