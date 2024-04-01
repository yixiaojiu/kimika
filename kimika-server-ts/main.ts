import grpc from '@grpc/grpc-js';
import remote_grpc from './src/proto/remote_grpc_pb';
import remote from './src/proto/remote_pb';

// const __dirname = path.dirname(new URL(import.meta.url).pathname);

// const PROTO_PATH = path.join(__dirname, '../kimika_grpc/proto/remote.proto');
// const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
//   keepCase: true,
//   longs: String,
//   enums: String,
//   defaults: true,
//   oneofs: true,
// });

// const protoDescriptor = grpc.loadPackageDefinition(packageDefinition);

// const remote = protoDescriptor.remote;

async function main() {
  const server = new grpc.Server();

  server.addService(remote_grpc.RemoteService, {});

  server.bindAsync('0.0.0.0:3940', grpc.ServerCredentials.createInsecure(), (err, port) => {
    console.log(`gRPC listening on ${port}`);
  });
}
