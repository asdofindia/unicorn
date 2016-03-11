# unicorn

"unicorn" is our sweet little name for **Unified Communications Over Real-time Networks**.

unicorn's purpose is to dissolve fragmentation of the internet; by
making it possible to bridge together different types of networks. It
aims to be a data-agnostic communications technology platform that can
connect any number and combination of clients (humans or machines).

The unicorn platform will use a federated architecture with
distributed points of discovery. Security will be baked into every
layer. It will work with technologies like WebRTC and inter-operate
with technologies like [Matrix](http://matrix.org), and extend
them to enhance interoperability of communication.

It aims to be agnostic of data-type. If the data type can be streamed,
unicorn will be able to support it. This means, it will work for
audio, video, text or binary data.

## Community

- [**Issues**](https://github.com/muktakosh/unicorn/issues)
- [**Forum**](https://muktakosh.org/c/unicorn)
- [**Telegram**](https://telegram.me/joinchat/AvJ4FgY8q5XVDqwHaaPOpQ)


## Roadmap

### v1.0

- Implement signaling of devices using the [Matrix specifications](http://matrix.org/docs/spec/r0.0.1/).
- Implement server federation using the [Matrix Federation specifications](http://matrix.org/docs/spec/r0.0.1/server_server.html).
- Core modules implementation in [Rust](https://www.rust-lang.org/).
- API Server implementation in NodeJS (Open to discussions)
- Client for connecting to unicorn servers, in JavaScript.

## License

This project's source code is subject to the terms of the
[Mozilla Public License, v. 2.0](https://mozilla.org/MPL/2.0).

Copyright (c) 2016 Muktakosh.org and [contributors](CREDITS).
