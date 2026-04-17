<h1 align="center">
    <a style="text-decoration: none" href="https://diom.svix.com">
      <img width="120" src="https://diom.svix.com/icon.svg" />
      <p align="center">Diom - Components for Robust Services</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://diom.svix.com">Website</a> | <a href="https://diom.svix.com/docs">Documentation</a> | <a href="https://svix.com/slack">Community Slack</a>
<h2>

![GitHub tag](https://img.shields.io/github/tag/svix/diom.svg)
[![Build Status](https://github.com/svix/diom/workflows/Server%20CI/badge.svg)](https://github.com/svix/diom/actions)
[![Server Security](https://github.com/svix/diom/actions/workflows/server-security.yml/badge.svg)](https://github.com/svix/diom/actions/workflows/server-security.yml)
[![Twitter Follow](https://img.shields.io/twitter/follow/SvixHQ?style=social)](https://twitter.com/SvixHQ)
[![Join our slack](https://img.shields.io/badge/Slack-join%20the%20community-blue?logo=slack&style=social)](https://www.svix.com/slack/)

## Diom is the backend components platform

Diom (pronounced: dye-omm/daɪəm) is a backend components platform for building robust, idiomatic services.

It offers high-level APIs for commonly used components such as cache, rate-limiting, idempotency, queue, and more. It has zero runtime dependencies, uses its own storage, and can be run as a single node or a highly-available cluster.


[![PyPI](https://img.shields.io/pypi/v/diom.svg)](https://pypi.python.org/pypi/diom/)
[![Crates.io](https://img.shields.io/crates/v/diom)](https://crates.io/crates/diom)
[![NPM version](https://img.shields.io/npm/v/@diomhq/diom.svg)](https://www.npmjs.com/package/@diomhq/diom)
[![Maven Central (Java)](https://img.shields.io/maven-central/v/com.svix/diom?label=maven-central%20(java))](https://search.maven.org/artifact/com.svix/diom)
[![PkgGoDev](https://pkg.go.dev/badge/github.com/svix/diom)](https://pkg.go.dev/diom.com/go/diom)


Useful links:

  - [Website](https://diom.svix.com) - the Diom homepage.
  - [Documentation](https://diom.svix.com/docs) - information on how to use Diom.
  - [GitHub Issues](https://github.com/svix/diom/issues) - report issues and make suggestions.
  - [Community Forum](https://github.com/svix/diom/discussions) - ask questions, and start discussions!
  - [Slack](https://www.svix.com/slack/) - come and chat with us!

To stay up-to-date with new features and improvements be sure to watch our repo!

## Interacting with Diom

Diom ships with client libraries for a variety of languages, as well as a CLI named `diom`.

<table style="table-layout:fixed; white-space: nowrap;">
  <th colspan="6">⚡️ Feature Breakdown ⚡️</th>
  <tr>
    <th>Language</th>
    <th>Officially Supported</th>
    <th colspan="4">Other Notes</th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/go/">Go</a></th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  </tr>
    <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/python/">Python</a></th>
    <th>✅</th>
    <th colspan="4">Sync and async.</th>
  </tr>
    </tr>
    <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/javascript/">TypeScript/JavaScript</a></th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/java/">Java</a></th>
    <th>✅</th>
    <th colspan="4">Async support planned.</th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/kotlin/">Kotlin</a></th>
    <th>🔜</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/ruby/">Ruby</a></th>
    <th>🔜</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/csharp/">C# (dotnet)</a></th>
    <th>🔜</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/rust/">Rust</a></th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/php/">PHP</a></th>
    <th>🔜</th>
    <th colspan="4"></th>
  </tr>
  <tr>
    <th><a href="https://github.com/svix/diom/tree/main/z-clients/cli/">CLI</a></th>
    <th>✅</th>
    <th colspan="4"></th>
  </tr>

</table>

## Running the server

Diom is designed to be run on a cluster of machines with stable network identities (e.g., a Kubernetes StatefulSet). It's recommended to run it as a three-node cluster for high-availability and easy operations, but it can also run as a single-node.

The `diom-server` binary is configured through a TOML file which can be passed with the `--config-path` command line option. Settings can also be overridden by setting environment variables.

## Server configuration

There are two main ways to configure `diom-server`: environment vars, and a configuration file.

Configuring the server is described in the [configuration section of the docs](https://diom.svix.com/docs/self-hosting/configuration).

## Observability using OpenTelemetry

Observability configuration described in the [observability section of the docs](https://diom.svix.com/docs/self-hosting/observability).


## Developing

This application is written in Rust and targets the latest stable release of Rust. You should install Rust with `rustup` or your favorite package manager. This application is broken up into multiple crates using [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

A [prek](https://prek.j178.dev/) config is included and all commits are expected to pass pre-commit checks (as well as more-intensive CI checks run through Github Actions). You should install `prek` and then configure it to run automatically on commits to this repo with `prek install`.

Many helpful commands are provided in the [justfile](.justfile) which can be invoked using [Just](https://github.com/casey/just), so you should make sure to have that installed too.

### Building

`cargo build` in the root should build the server by default. If you want the local CLI, you'll also need `cargo build --package diom-cli`

### Making changes

Changes should be done in branches prefixed with your username (e.g., `johnsmith/my-cool-feature`) and should have commit messages that describe the change. Prior to sending any PRs, commits must pass `prek`, `just lint`, and `just test`. Any changes that affect the client libraries / CLI will require running `just codegen` to rebuild all client libraries.
