import * as grpc from '@grpc/grpc-js';
import remote_pb from './proto/remote_pb';
import { nanoid } from 'nanoid';
import { receiverMap, contentMap } from './state';
import { onReceiver, emitReceiver, onContent, emitContent } from './events';

function registerReceiver(
  call: grpc.ServerUnaryCall<remote_pb.RegisterReceiverRequest, remote_pb.RegisterResponse>,
  callback: grpc.sendUnaryData<remote_pb.RegisterResponse>
) {
  const body = call.request.toObject();
  const ip = call.getPeer().split(':')[0];
  let res = new remote_pb.RegisterResponse();
  let id = nanoid();
  receiverMap.set(id, { ...body, ip });
  emitReceiver();
  res.setId(id);
  callback(null, res);
}

function registerContent(
  call: grpc.ServerUnaryCall<remote_pb.RegisterContentRequest, remote_pb.RegisterResponse>,
  callback: grpc.sendUnaryData<remote_pb.RegisterResponse>
) {
  const content = call.request.toObject();
  const ip = call.getPeer().split(':')[0];
  let res = new remote_pb.RegisterResponse();
  let id = nanoid();
  contentMap.set(id, { ...content, ip });
  res.setId(id);
  callback(null, res);
}

async function getReceivers(call: grpc.ServerWritableStream<remote_pb.EmptyRequest, remote_pb.GetReceiversResponse>) {
  while (true) {
    if (!call.writable) {
      break;
    }
    let res = new remote_pb.GetReceiversResponse();
    const receivers = Array.from(receiverMap.values());
    res.setReceiversList(
      receivers.map(item => {
        const receiverRes = new remote_pb.GetReceiversResponse.Receiver();
        receiverRes.setAlias(item.alias);
        receiverRes.setIp(item.ip);
        return receiverRes;
      })
    );
    call.write(res);
    await onReceiver();
  }
  call.end();
}

async function getContent(call: grpc.ServerWritableStream<remote_pb.EmptyRequest, remote_pb.GetContentReponse>) {
  while (true) {
    if (!call.writable) {
      break;
    }
    // let res = new remote_pb.GetContentReponse();
    // const content = contentMap.get(call.request.getId());
    // if (content) {
    //   res.setContentType(content.contentType);
    //   res.setAlias(content.alias);
    //   res.setIp(content.ip);
    //   res.setSize(content.size || 0);
    //   res.setName(content.name || '');
    // }
    // call.write(res);
    // await onContent();
  }
  call.end();
}

export default {
  registerReceiver,
  registerContent,
  getReceivers,
};
