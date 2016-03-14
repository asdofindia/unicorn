# Contributing to unicorn

## Build

### Pre-requisites

- [Rust](https://www.rust-lang.org/) v1.7.0+ with `cargo`.

### Building from source

- Clone the repo
- At the root of the repo, run:

      ```
      $ cargo build
      ```

This will create the compiled binary at `./target/debug/unicorn`.

### Creating a release build

If you want a build with all optimizations in place, run this at the root of the repo:

```
$ cargo build --release
```

This will create the compiled binary at `./target/release/unicorn`.

## Tests

```
$ cargo test
```
