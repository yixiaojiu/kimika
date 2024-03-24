# 记录

## 功能

### 配置文件

```toml
[profile]
# 用于标识
alias = ""

[server]
# 默认使用 server 桥接
enable = false
# 地址
address = 127.0.0.1:3939
```

### 发送消息（未指定目标）

1. `kimika receive`
2. 接收端等待接收 udp 广播包，并开启 grpc 服务
3. `kimika send -m "hello"`
4. 发送端 udp 广播寻找接收者
5. 接收端回应 udp 包，告知 IP，端口与别名
6. 发送端选择接收对象，并与接收端建立 grpc 连接。接收端停止监听 udp
7. 发送端发送数据

### 发送消息（指定目标）

1. `kimika receive`
2. 接收端等待接收 udp 广播包，并开启 grpc 服务
3. `kimika send -m "hello" --target 127.0.0.1:3000`
4. 发送端连接 grpc。接收端停止监听 udp
5. 发送端发送数据

### 发送消息（通过服务器）

1. 服务器开启 grpc 服务
2. `kimika receive --server` 接收端向服务器注册（别名，IP），并建立服务器向发送端的单向流
3. `kimika send -m "hello" --server` 发送端向服务器查询有哪些接收端
4. 选择接收端，发送数据，服务器将数据转发给接收端
