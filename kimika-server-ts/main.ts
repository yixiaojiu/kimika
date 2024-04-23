import remote_grpc from './src/proto/remote_grpc_pb';
// cjs import
import * as grpc from '@grpc/grpc-js';
import handler from './src/handler';
import { CronJob } from 'cron';
import { checkState } from './src/state';
import { loggingInterceptor } from './src/middleware';

new CronJob(
  '0 0 * * * *',
  function () {
    checkState('receive');
    checkState('content');
  },
  null,
  true,
  'America/Los_Angeles'
);

function main() {
  const server = new grpc.Server({
    interceptors: [loggingInterceptor],
  });

  server.addService(remote_grpc.RemoteService, handler);

  server.bindAsync('0.0.0.0:3941', grpc.ServerCredentials.createInsecure(), (err, port) => {
    if (err != null) {
      return console.error(err);
    }
  });
}

main();
