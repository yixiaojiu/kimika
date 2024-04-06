import remote_grpc from './src/proto/remote_grpc_pb';
// cjs import
import * as grpc from '@grpc/grpc-js';
import handler from './src/handler';

function main() {
  const server = new grpc.Server();

  server.addService(remote_grpc.RemoteService, handler);

  server.bindAsync('0.0.0.0:3940', grpc.ServerCredentials.createInsecure(), (err, port) => {
    if (err != null) {
      return console.error(err);
    }
    console.log(`gRPC listening on ${port}`);
  });
}

main();
