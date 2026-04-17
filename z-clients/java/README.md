<h1 align="center">
    <a style="text-decoration: none" href="https://www.svix.com">
      <img width="120" src="https://diom.svix.com/icon.svg" />
      <p align="center">Diom - by Svix</p>
    </a>
</h1>


Java library for interacting with the Diom API

![GitHub tag](https://img.shields.io/github/tag/svix/diom.svg)
[![Maven Central (Java)](https://img.shields.io/maven-central/v/com.svix/diom?label=maven-central%20(java))](https://search.maven.org/artifact/com.svix/diom)


# Usage Documentation

You can find general usage documentation at <https://diom.svix.com/docs>.


# Installation

### Maven users

Add this dependency to your project's POM:

```xml
<dependency>
  <groupId>com.svix</groupId>
  <artifactId>diom</artifactId>
  <version>0.2.1</version>
  <scope>compile</scope>
</dependency>
```

### Gradle users

Add this dependency to your project's build file:

```groovy
implementation "com.svix:diom:0.2.1"
```

## Usage
Please refer to [the documentation](https://diom.svix.com) for more usage instructions.

# Development

First checkout the [core README](../../README.md#developing) for details on how to generate our API bindings, then follow the steps below.

## Requirements

 - Java 8+
 - Maven

## Building the library
```sh
mvn package
```

## Running Tests

Simply run:

```sh
mvn test
```
