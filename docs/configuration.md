# Configuration

Configuration file location: `~/.config/kimika/config.toml`

Here is a example configuration:

```toml
alias = "kimika"

[receiver]
save_folder = "./"
port = 3940

[sender]
port = 3939
receiver_port = 3940

[[server]]
alias = "local"
address = "127.0.0.1:3941"
```

## `alias`

Alias used for identification

## [receiver]

### `save_folder`

The folder to save when a file received

### `port`

The listening port when running `kimika receive`

## [sender]

### `receiver_port`

Receiver listening port when transfer from local network

### `port`

The listening port when running `kimika send`

## [[server]]

This is array of tables, you can define multiple servers.

### `alias`

Alias used to identify the server

### `address`

The address of the server
