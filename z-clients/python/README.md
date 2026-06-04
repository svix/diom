<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://diom.svix.com/icon.svg" />
      <p align="center">Diom - by Svix</p>
    </a>
</h1>


Python library for interacting with the Diom API

![GitHub tag](https://img.shields.io/github/tag/svix/diom.svg)
[![PyPI](https://img.shields.io/pypi/v/diom.svg)](https://pypi.python.org/pypi/diom/)

# Usage Documentation

You can find general usage documentation at <https://diom.svix.com/docs>.

# Installation

```sh
pip install diom
```

This library supports all [supported versions](https://devguide.python.org/versions/) of Python.

## Usage
Please refer to [the documentation](https://diom.svix.com) for more usage instructions.

# Development

First checkout the [core README](../../README.md#developing) for details on how to generate our API bindings, then follow the steps below.

## Requirements

 - python >= 3.10

## Installing dependencies

We recommend using [uv](https://github.com/astral-sh/uv) for working on this library itself.

```sh
uv sync
```

## Contributing

Before opening a PR be sure to format your code!

```sh
uv run ruff format
```

## Running Tests

Simply run:

```sh
uv run pytest
```

If you want to run integration tests, you'll need to start the Diom server and set the `$DIOM_TOKEN` and `$DIOM_SERVER_URL` environment variables accordingly.
