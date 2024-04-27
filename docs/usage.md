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

## Transfer in local network

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

## Transfer through remote server
