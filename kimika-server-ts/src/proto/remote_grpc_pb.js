// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var remote_pb = require('./remote_pb.js');

function serialize_remote_ChooseReceiverRequest(arg) {
  if (!(arg instanceof remote_pb.ChooseReceiverRequest)) {
    throw new Error('Expected argument of type remote.ChooseReceiverRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_ChooseReceiverRequest(buffer_arg) {
  return remote_pb.ChooseReceiverRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_ChooseReceiverResponse(arg) {
  if (!(arg instanceof remote_pb.ChooseReceiverResponse)) {
    throw new Error('Expected argument of type remote.ChooseReceiverResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_ChooseReceiverResponse(buffer_arg) {
  return remote_pb.ChooseReceiverResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_EmptyRequest(arg) {
  if (!(arg instanceof remote_pb.EmptyRequest)) {
    throw new Error('Expected argument of type remote.EmptyRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_EmptyRequest(buffer_arg) {
  return remote_pb.EmptyRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_EmptyResponse(arg) {
  if (!(arg instanceof remote_pb.EmptyResponse)) {
    throw new Error('Expected argument of type remote.EmptyResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_EmptyResponse(buffer_arg) {
  return remote_pb.EmptyResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_GetContentReponse(arg) {
  if (!(arg instanceof remote_pb.GetContentReponse)) {
    throw new Error('Expected argument of type remote.GetContentReponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_GetContentReponse(buffer_arg) {
  return remote_pb.GetContentReponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_GetReceiversResponse(arg) {
  if (!(arg instanceof remote_pb.GetReceiversResponse)) {
    throw new Error('Expected argument of type remote.GetReceiversResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_GetReceiversResponse(buffer_arg) {
  return remote_pb.GetReceiversResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_ReceiveRequest(arg) {
  if (!(arg instanceof remote_pb.ReceiveRequest)) {
    throw new Error('Expected argument of type remote.ReceiveRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_ReceiveRequest(buffer_arg) {
  return remote_pb.ReceiveRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_RegisterContentRequest(arg) {
  if (!(arg instanceof remote_pb.RegisterContentRequest)) {
    throw new Error('Expected argument of type remote.RegisterContentRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_RegisterContentRequest(buffer_arg) {
  return remote_pb.RegisterContentRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_RegisterReceiverRequest(arg) {
  if (!(arg instanceof remote_pb.RegisterReceiverRequest)) {
    throw new Error('Expected argument of type remote.RegisterReceiverRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_RegisterReceiverRequest(buffer_arg) {
  return remote_pb.RegisterReceiverRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_RegisterResponse(arg) {
  if (!(arg instanceof remote_pb.RegisterResponse)) {
    throw new Error('Expected argument of type remote.RegisterResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_RegisterResponse(buffer_arg) {
  return remote_pb.RegisterResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_TransferContent(arg) {
  if (!(arg instanceof remote_pb.TransferContent)) {
    throw new Error('Expected argument of type remote.TransferContent');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_TransferContent(buffer_arg) {
  return remote_pb.TransferContent.deserializeBinary(new Uint8Array(buffer_arg));
}


var RemoteService = exports.RemoteService = {
  registerReceiver: {
    path: '/remote.Remote/RegisterReceiver',
    requestStream: false,
    responseStream: false,
    requestType: remote_pb.RegisterReceiverRequest,
    responseType: remote_pb.RegisterResponse,
    requestSerialize: serialize_remote_RegisterReceiverRequest,
    requestDeserialize: deserialize_remote_RegisterReceiverRequest,
    responseSerialize: serialize_remote_RegisterResponse,
    responseDeserialize: deserialize_remote_RegisterResponse,
  },
  registerContent: {
    path: '/remote.Remote/RegisterContent',
    requestStream: false,
    responseStream: false,
    requestType: remote_pb.RegisterContentRequest,
    responseType: remote_pb.RegisterResponse,
    requestSerialize: serialize_remote_RegisterContentRequest,
    requestDeserialize: deserialize_remote_RegisterContentRequest,
    responseSerialize: serialize_remote_RegisterResponse,
    responseDeserialize: deserialize_remote_RegisterResponse,
  },
  getContent: {
    path: '/remote.Remote/GetContent',
    requestStream: false,
    responseStream: true,
    requestType: remote_pb.EmptyRequest,
    responseType: remote_pb.GetContentReponse,
    requestSerialize: serialize_remote_EmptyRequest,
    requestDeserialize: deserialize_remote_EmptyRequest,
    responseSerialize: serialize_remote_GetContentReponse,
    responseDeserialize: deserialize_remote_GetContentReponse,
  },
  getReceivers: {
    path: '/remote.Remote/GetReceivers',
    requestStream: false,
    responseStream: true,
    requestType: remote_pb.EmptyRequest,
    responseType: remote_pb.GetReceiversResponse,
    requestSerialize: serialize_remote_EmptyRequest,
    requestDeserialize: deserialize_remote_EmptyRequest,
    responseSerialize: serialize_remote_GetReceiversResponse,
    responseDeserialize: deserialize_remote_GetReceiversResponse,
  },
  chooseReceiver: {
    path: '/remote.Remote/ChooseReceiver',
    requestStream: false,
    responseStream: true,
    requestType: remote_pb.ChooseReceiverRequest,
    responseType: remote_pb.ChooseReceiverResponse,
    requestSerialize: serialize_remote_ChooseReceiverRequest,
    requestDeserialize: deserialize_remote_ChooseReceiverRequest,
    responseSerialize: serialize_remote_ChooseReceiverResponse,
    responseDeserialize: deserialize_remote_ChooseReceiverResponse,
  },
  send: {
    path: '/remote.Remote/Send',
    requestStream: true,
    responseStream: false,
    requestType: remote_pb.TransferContent,
    responseType: remote_pb.EmptyResponse,
    requestSerialize: serialize_remote_TransferContent,
    requestDeserialize: deserialize_remote_TransferContent,
    responseSerialize: serialize_remote_EmptyResponse,
    responseDeserialize: deserialize_remote_EmptyResponse,
  },
  receive: {
    path: '/remote.Remote/Receive',
    requestStream: false,
    responseStream: true,
    requestType: remote_pb.ReceiveRequest,
    responseType: remote_pb.TransferContent,
    requestSerialize: serialize_remote_ReceiveRequest,
    requestDeserialize: deserialize_remote_ReceiveRequest,
    responseSerialize: serialize_remote_TransferContent,
    responseDeserialize: deserialize_remote_TransferContent,
  },
};

exports.RemoteClient = grpc.makeGenericClientConstructor(RemoteService);
