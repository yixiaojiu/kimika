# Usage

```
Usage: kimika <COMMAND>

Commands:
  send     send file
  receive  receive file or message
  help     Print this message or the help of the given subcommand(s)
```

```
Usage: kimika receive [OPTIONS]

Options:
      --port <port>        listen port
  -f, --folder <folder>    save folder
      --alias <alias>      alias used for identification
  -s, --server             whether to use remote server
  -a, --address <address>  remote server address. Such as: example.com
```

```
Usage: kimika send [OPTIONS]

Options:
  -p, --path <path>                    the path of file which want to send
  -m, --message <message>              text which wants to send
  -a, --address <address>              receiver address or remote server address. Such as: example.com
      --port <port>                    listen port
      --receiver-port <receiver_port>  receiver port when transfer from local network
      --alias <alias>                  alias used for identification
  -s, --server                         whether to use remote server
```

## Sender

Send a text through local network

```sh
kimika send -m "hello world"
```

Send a file through local network

```sh
kimika send -p demo.txt
```

Use remote server

First of all, you should have a remote server. Follow [docs/server](/docs/server.md) to deploy a server.

Assume you have a server with the address `127.0.0.1:3941`

```sh
kimika send -m "hello world" -s -a 127.0.0.1:3941
```

If you set the server in the configuration file, you do not need to specify the address.

For example: `~/.config/kimika/config.toml`

```toml
[[server]]
alias = "local"
address = "127.0.0.1:3941"
```

```sh
kimika send -m "hello world" -s
```

For configuration file details, please refer to [docs/configuration](/docs/configuration.md)

## Receiver

Receive in local network

```sh
kimika receive
```

Receive through remote server

```sh
kimika receive -s -a 127.0.0.1:3941
```

You can also don't specify the address, if you set the server in the configuration file.

## Use with pipeline

Such as: Using with [wl-clipboard](https://github.com/bugaevc/wl-clipboard)

```sh
wl-paste | kimika send
```

```sh
kimika receive | wl-copy
```
