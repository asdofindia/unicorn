# Contributing to unicorn

## Public domain declaration

unicorn is free and unencumbered software released into the public domain. We can only accept your contributions if you dedicate it to the public domain as per the clauses of the [LICENSE](LICENSE). We request you to please sign the declaration mentioned in [CREDITS](credits.md) by adding your name and email to the list of contributors as part of your patch.

Please refrain from contributing patches that conflict with the LICENSE or that you do not own the right to dedicate to public domain.

## Build Instructions

### Pre-requisites

- Install [Rust](https://www.rust-lang.org/) `stable` (v1.9.0+) with `cargo`. We recommend using [`rustup`](https://rustup.rs).
- Install Make.
- Clone the repo.

#### GNU/Linux

- Set the `LD_LIBRARY_PATH` environment variable:

    ```sh
    export LD_LIBRARY_PATH=/usr/local/lib
    ```

- Set-up dependencies:

    ```
    $ make deps-linux
    ```

#### OSX

- On OSX, add `LIBRARY_PATH` environment variable:

    ```sh
    export LIBRARY_PATH="/usr/local/lib"
    ```

- Set-up dependencies:

    ```
    $ make deps
    ```

### Trigger build


To build the project in `debug` mode, run:

```
$ make build
```

This will create the compiled binary at `./target/debug/unicorn`.

### Creating a release build

If you want a build with all optimizations in place, run this at the root of the repo:

```
$ make build-release
```

This will create the compiled binary at `./target/release/unicorn`.

### Build against nightly Rust

If you want to build against nightly Rust, install `rustup` with `nightly` toolchain and run:

```
$ make nightly-build
```

This will create the compiled binary at `./target/release/unicorn`.

## Tests

To execute tests, run:

```
$ make test
```
