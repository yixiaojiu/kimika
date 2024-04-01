# kimika-server-ts

grpc_tools_node_protoc \
--js_out=import_style=commonjs,binary:./src \
--grpc_out=grpc_js:./src \
--plugin=protoc-gen-grpc=`which grpc_tools_node_protoc_plugin` \
-I ./proto \
./proto/remote.proto

protoc \
--plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
--ts_out=grpc_js:./src \
-I ./proto \
./proto/remote.proto
