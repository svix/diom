# Changelog

## Unreleased Changes

## Version 0.2.4
* CLI: Remove underlining from schema examples on `--help` messages due to portability issues
* Msgs: Add lease cancellation operation
* Server: Various clustering improvements
* Libs/Rust: Don't require optional fields

### Breaking Changes
* Libs/Rust: Revamp `ErrorKind` and error type methods

## Version 0.2.3
* Server: Several configuration values that were specified as millisecond durations are now explicitly checked for being non-zero at startup
* Libs/Rust: expose `.is_retryable()` and `.kind()` on `diom::Error`
* Libs/Rust: do not leak feature `release_max_level_debug` into the tracing library
* Libs/All: remove automatic retries
* Miscellaneous dependency bumps
* Various improvements to release infrastructure

### Breaking Changes
* `bootstrap_cfg_path` is replaced by `bootstrap_cfg_paths` (an array). `bootstrap_cfg` (inline) and `bootstrap_cfg_paths` can now both be set; inline is applied first. `$DIOM_BOOTSTRAP_CFG_PATH` is replaced by `$DIOM_BOOTSTRAP_CFG_PATHS`.

## Version 0.2.2
* More build & release fixes

## Version 0.2.1
* Fix Rust build

## Version 0.2.0
* Initial server release.
* Initial real library release.
