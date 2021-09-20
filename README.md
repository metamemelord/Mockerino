# Mockerino
 [![Docker build](https://github.com/metamemelord/Mockerino/actions/workflows/build.yml/badge.svg)](https://github.com/metamemelord/Mockerino/actions/workflows/build.yml)
 
A YAML based REST API engine.

## Features of Mockerino
* Supports kubernetes-like YAML spec file.
* Directory structure does routing.
* Supports response body from file or raw body.
* Supports adding response headers.
* Supports using custom response status code.
* Supports limiting max threads used by the server.
* Provides option to sleep before replying to request.

## Use-cases
* Use as a mocking engine for large systems without the need of writing code.
* Serve static assets.

## How to install Mockerino
* Download the binary from [releases page](https://github.com/metamemelord/Mockerino/releases).
* Build from sources
	+ Requires Rust 1.54+ and libssl-dev (only on Linux).
	+ Run `cargo build --release`
	+ (Optionally) The built binary contains ELF Symbols. These can be stripped by using `strip mockerino` command.
	
## Usage
* To initialize a sample spec run `mockerino init`. `config.yaml` file, `spec`, and `data` directories are created in the working directory.
* `./mockerino` starts the engine.

## Planned features
* Support for dynamic paths using path parameters and wildcard spec files.
