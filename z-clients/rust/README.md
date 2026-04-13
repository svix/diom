<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">Diom - by Svix</p>
    </a>
</h1>

Rust library for interacting with the Diom API

![GitHub tag](https://img.shields.io/github/tag/svix/diom.svg)
[![Crates.io](https://img.shields.io/crates/v/diom)](https://crates.io/crates/diom)
[![docs.rs](https://docs.rs/diom/badge.svg)](https://docs.rs/diom/)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

# Usage Documentation

You can find general usage documentation at <https://diom.svix.com/docs>.


## Usage
Please refer to [the documentation](https://diom.svix.com) for more usage instructions.

# Optional Cargo Features

## TLS

By default the client uses `rustls` with native certificate roots.

Native TLS (OpenSSL on Linux, SChannel on Windows, Secure Transport on macOS) can also be chosen:
```
diom = { version = "N", features = ["native-tls"], default-features = false }
```

## HTTP version

Both HTTP/1.1 and HTTP/2 are supported. HTTP/1.1 is enabled by default:
```
diom = { version = "N", features = ["http2"] }
```
