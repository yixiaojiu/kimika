import remote_grpc from './src/proto/remote_grpc_pb';
import remote from './src/proto/remote_pb';
// cjs import
import * as grpc from '@grpc/grpc-js';

// function getReceivers(call: grpc.Call, callback: any) {
//   const receivers = new remote.GetReceiversResponse();
//   receivers.addReceivers();
//   callback(null, receivers);
// }

function register(call: grpc.ServerUnaryCall<remote.RegisterRequest, remote.RegisterResponse>, callback: any) {
  // call.request
  const res = new remote.RegisterResponse();
  res.setContentType(remote.RegisterResponse.Type.MESSAGE);
  res.setId('bar');
  callback(null, res);
}

async function main() {
  const server = new grpc.Server();

  server.addService(remote_grpc.RemoteService, { register });

  server.bindAsync('0.0.0.0:3940', grpc.ServerCredentials.createInsecure(), (err, port) => {
    if (err != null) {
      return console.error(err);
    }
    console.log(`gRPC listening on ${port}`);
  });
}

main();
