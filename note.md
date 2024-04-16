# 记录

## 功能

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
2. `kimika receive --server`
3. 接收端向服务器调用`RegisterReceiver`注册，服务器返回 id
4. 接收端向服务器调用 `GetContent`，服务器返回数据流
5. `kimika send -m "hello" --server` 发送端向服务器查询有哪些接收端，服务端返回数据流
6. 发送端调用 `RegisterContent` 注册需要发送的内容，服务器返回 id
7. 发送端调用 `GetReceivers`，服务器返回数据流
8. 发送端选择接收端，调用 `ChooseReceiver`，服务器返回 `GetContent` 响应
9. 发送端调用 `Receive`，传入 `Content id`，服务端响应 `ChooseReceiver`，返回响应的 `Content id`
10. 发送端调用 `Send`
