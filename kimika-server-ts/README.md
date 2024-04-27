# kimika-server-ts

```sh
grpc_tools_node_protoc \
--js_out=import_style=commonjs,binary:./src/proto \
--grpc_out=grpc_js:./src/proto \
--plugin=protoc-gen-grpc=`which grpc_tools_node_protoc_plugin` \
-I ./proto \
./proto/remote.proto
```

```sh
protoc \
--plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
--ts_out=grpc_js:./src/proto \
-I ./proto \
./proto/remote.proto
```
