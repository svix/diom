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

## Usage
Please refer to [the documentation](https://diom.svix.com) for more usage instructions.

# Development

First checkout the [core README](../../README.md#developing) for details on how to generate our API bindings, then follow the steps below.

## Requirements

 - python >= 3.12

## Installing dependencies

```sh
python -m venv .venv
pip install -e ".[dev]"
```

## Contributing

Before opening a PR be sure to format your code!

```sh
ruff format
```

## Running Tests

Simply run:

```sh
pytest
```
