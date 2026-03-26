# Coyote

This is a multimodal, clustered database being developed by [Svix](https://www.svix.com). For more information, see its project website at <https://coyote.svix.io>.

## Running Coyote

Coyote is designed to be run on a cluster of machines with stable network identities (e.g., a Kubernetes StatefulSet). It can also be run in a single-node configuration, although this is not recommended for a production deployment.

Coyote is configured through a TOML file which can be passed with the `--config-path` command line option. Settings can also be overridden by setting environment variables.

## Interacting with Coyote

Coyote ships with client libraries for Rust, Python, Go, Java, and JavaScript, as well as a CLI named `coyote-cli`.

## Developing

This application is written in Rust and targets the latest stable release of Rust. You should install Rust with `rustup` or your favorite package manager. This application is broken up into multiple crates using [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

A [prek](https://prek.j178.dev/) config is included and all commits are expected to pass pre-commit checks (as well as more-intensive CI checks run through Github Actions). You should install `prek` and then configure it to run automatically on commits to this repo with `prek install`.

Many helpful commands are provided in the [justfile](.justfile) which can be invoked using [Just](https://github.com/casey/just), so you should make sure to have that installed too.

### Building

`cargo build` in the root should build the server by default. If you want the local CLI, you'll also need `cargo build --package coyote-cli`

### Making changes

Changes should be done in branches prefixed with your username (e.g., `johnsmith/my-cool-feature`) and should have commit messages that describe the change. Prior to sending any PRs, commits must pass `prek`, `just lint`, and `just test`. Any changes that affect the client libraries / CLI will require running `just codegen` to rebuild all client libraries.
