<div align="center">
  <img src="assets/kimika.png" alt="Yazi logo" width="200">
</div>

## Kimika - CLI tool for sending text or files

## Features

- automatically search receivers
- support pipeline
- based on grpc

## Installation

```sh
git clone https://github.com/yixiaojiu/kimika

cargo install ./kimika/kimika
```

## Usage

### Send text

On the one device, run the following command:

```sh
kimika send -m "hello world"
```

On the other device

```sh
kimika receive
```

### Send file

On the one device, run the following command:

```sh
kimika send -p demo.txt
```

On the other device

```sh
kimika receive
```

## How to develop

Follow [tonic getting started](https://github.com/hyperium/tonic?tab=readme-ov-file#getting-started) to install `protobuf complier`
