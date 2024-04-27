<div align="center">
  <img src="assets/kimika.png" alt="Yazi logo" width="200">
</div>

## Kimika - CLI tool for sending text or files

Kimika is a CLI tool for sending text or files written in Rust, based on grpc. It aims to get rid of dependence on wechat or QQ.

- 🌟 Automatically search receivers
- 💫 Support pipeline
- 🖼️ Support transfer through remote server
- 💡 Support transfer through local network
- 💪 Based on grpc

![demo](assets/demo.gif)

## Installation

Follow [tonic getting started](https://github.com/hyperium/tonic?tab=readme-ov-file#dependencies) to install `protobuf complier`

```sh
cargo install --git https://github.com/yixiaojiu/kimika kimika
```

## Document

- [docs/usage](/docs/usage.md)
- [docs/server](/docs/server.md)
- [docs/configuration](/docs/configuration.md)
