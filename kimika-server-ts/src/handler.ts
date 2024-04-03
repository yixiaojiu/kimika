import * as grpc from '@grpc/grpc-js';
import remote_pb from './proto/remote_pb';
import { nanoid } from 'nanoid';
import { receiverMap, contentMap, senderMap } from './state';
import { onReceiver, emitReceiver, onContent, emitContent, onSender, emitSender, onStream, emitStream } from './events';

function registerReceiver(
  call: grpc.ServerUnaryCall<remote_pb.RegisterReceiverRequest, remote_pb.RegisterReceiverResponse>,
  callback: grpc.sendUnaryData<remote_pb.RegisterReceiverResponse>
) {
  const body = call.request.toObject();
  const ip = call.getPeer().split(':')[0];
  let res = new remote_pb.RegisterReceiverResponse();
  let id = nanoid();
  receiverMap.set(id, { ...body, ip, id });
  emitReceiver();
  res.setReceiverId(id);
  callback(null, res);
}

function registerContent(
  call: grpc.ServerUnaryCall<remote_pb.RegisterContentRequest, remote_pb.RegisterContentResponse>,
  callback: grpc.sendUnaryData<remote_pb.RegisterContentResponse>
) {
  const content = call.request.toObject();
  const ip = call.getPeer().split(':')[0];
  let res = new remote_pb.RegisterContentResponse();
  let contentId = nanoid();
  const senderId = nanoid();
  contentMap.set(contentId, { ...content, ip, senderId, contentId });
  senderMap.set(senderId, [contentId]);
  res.setContentId(contentId);
  res.setSenderId(senderId);
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
        receiverRes.setReceiverId(item.id);
        return receiverRes;
      })
    );
    call.write(res);
    await onReceiver();
  }
  call.end();
}

async function getContent(call: grpc.ServerWritableStream<remote_pb.GetContentRequest, remote_pb.GetContentReponse>) {
  const receiver_id = call.request.getReceiverId();
  while (true) {
    const senderId = await onContent(receiver_id);
    if (!call.writable) {
      break;
    }

    const contentIdList = senderMap.get(senderId);
    if (!contentIdList) {
      break;
    }

    const contentList = contentIdList.map(item => contentMap.get(item)!);

    const res = new remote_pb.GetContentReponse();
    res.setContentListList(
      contentList.map(item => {
        const contentRes = new remote_pb.GetContentReponse.Content();
        contentRes.setContentType(item.contentType);
        contentRes.setAlias(item.alias);
        contentRes.setIp(item.ip);
        contentRes.setSenderId(item.senderId);
        contentRes.setContentId(item.contentId);
        if (item.size) {
          contentRes.setSize(item.size);
        }
        if (item.name) {
          contentRes.setName(item.name);
        }
        return contentRes;
      })
    );
    call.write(res);
  }
  call.end();
}

async function chooseReceiver(
  call: grpc.ServerWritableStream<remote_pb.ChooseReceiverRequest, remote_pb.ChooseReceiverResponse>
) {
  const receiverId = call.request.getReceiverId();
  const senderId = call.request.getSenderId();
  emitContent(receiverId, senderId);

  while (true) {
    const contentId = await onSender(receiverId);
    if (!call.writable) {
      break;
    }
    const res = new remote_pb.ChooseReceiverResponse();
    res.setContentId(contentId);
    res.setReceiverId(receiverId);
    call.write(res);
  }
  call.end();
}

async function receive(call: grpc.ServerWritableStream<remote_pb.ReceiveRequest, remote_pb.TransferContent>) {
  const contentId = call.request.getContentId();
  const receiverId = call.request.getReceiverId();
  emitSender(receiverId, contentId);
  const stream = await onStream(contentId);
  stream.on('data', (chunk: remote_pb.TransferContent) => {
    call.write(chunk);
  });

  stream.on('end', () => {
    call.end();
  });
}

async function send(
  call: grpc.ServerReadableStream<remote_pb.TransferContent, remote_pb.EmptyResponse>,
  callback: grpc.sendUnaryData<remote_pb.EmptyResponse>
) {
  const contentId = call.metadata.get('id')[0] as string;
  emitStream(contentId, call as any);
  call.on('end', () => {
    const res = new remote_pb.EmptyResponse();
    callback(null, res);
  });
}

export default {
  registerReceiver,
  registerContent,
  getReceivers,
  getContent,
  chooseReceiver,
  receive,
  send,
};
