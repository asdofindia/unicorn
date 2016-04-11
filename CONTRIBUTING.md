# Contributing to unicorn

## Public domain declaration

unicorn is free and unencumbered software released into the public domain. We can only accept your contributions if you dedicate it to the public domain as per the clauses of the [LICENSE](LICENSE). We request you to please sign the declaration mentioned in [CREDITS](credits.md) by adding your name and email to the list of contributors as part of your patch.

Please refrain from contributing patches that conflict with the LICENSE or that you do not own the right to dedicate to public domain.

## Build

### Pre-requisites

- [Rust](https://www.rust-lang.org/) v1.7.0+ with `cargo`.

#### Debian/Ubuntu

```sh
sudo apt-get install libzmq3-dev
```

#### All GNU/Linux

Download latest [libzmq stable release](http://download.zeromq.org/zeromq-4.1.4.tar.gz) and install libsodium.

```sh
# Install libsodium before the next step
./configure --with-libsodium
make check
sudo make install
sudo ldconfig
```

#### OSX

Add `LIBRARY_PATH` environment variable:

```sh
export LIBRARY_PATH="/usr/local/lib"
```

Install `zeromq` from Homebrew:

```sh
brew install zeromq --with-libpgm --with-libsodium
```

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