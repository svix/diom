# Changelog

See also [the diom-operator ChangeLog](./infra/operator/ChangeLog.md) for changes to the Kubernetes infrastructure.

## Unreleased Changes

## Version 0.2.5
* Server: Add `/api/v1.msgs.topic.list` operation to list all topics
* Server: Add the ability to subscribe to a Svix AutoConfig endpoint and append messages from it to a Diom topic
* Server: Add the ability to automatically emit messages from a Diom topic to an arbitrary HTTP server
* Server: Add a set of endpoints exposing the current values of all OpenTelemetry metrics, for polling
* Server: Make body size limit configurable with `max_body_size`
* Server: Improve reliability of cluster bootstrapping and snapshots; snapshots now include Cluster UUID metadata
* Server: Add a dedicated "readiness" endpoint
* Server: Expose some internal Tokio metrics to OpenTelemetry (thanks @KaustubhOG)
* CLI: Ignore `EPIPE` to make it easier to use the CLI in shell pipelines
* CLI: Allow parsing input as JSONC in addition to JSON (thanks @KaustubhOG)
* CLI: Allow loading a JSON argument from a file by specifying it prefixed with the `@` argument on the command-line;
  e.g., `diom kv set key value @options.json` to load the options from `options.json`. Note that this can only be used for arguments
  which take a JSON body (where the leading `@` is unambiguous) and cannot currently be used for strings like K/V keys or values.

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
