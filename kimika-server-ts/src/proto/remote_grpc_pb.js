// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var remote_pb = require('./remote_pb.js');

function serialize_remote_EmptyResponse(arg) {
  if (!(arg instanceof remote_pb.EmptyResponse)) {
    throw new Error('Expected argument of type remote.EmptyResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_EmptyResponse(buffer_arg) {
  return remote_pb.EmptyResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_GetReceiversRequest(arg) {
  if (!(arg instanceof remote_pb.GetReceiversRequest)) {
    throw new Error('Expected argument of type remote.GetReceiversRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_GetReceiversRequest(buffer_arg) {
  return remote_pb.GetReceiversRequest.deserializeBinary(new Uint8Array(buffer_arg));
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

function serialize_remote_ReceiveResponse(arg) {
  if (!(arg instanceof remote_pb.ReceiveResponse)) {
    throw new Error('Expected argument of type remote.ReceiveResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_ReceiveResponse(buffer_arg) {
  return remote_pb.ReceiveResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_remote_RegisterRequest(arg) {
  if (!(arg instanceof remote_pb.RegisterRequest)) {
    throw new Error('Expected argument of type remote.RegisterRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_RegisterRequest(buffer_arg) {
  return remote_pb.RegisterRequest.deserializeBinary(new Uint8Array(buffer_arg));
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

function serialize_remote_SendRequest(arg) {
  if (!(arg instanceof remote_pb.SendRequest)) {
    throw new Error('Expected argument of type remote.SendRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_remote_SendRequest(buffer_arg) {
  return remote_pb.SendRequest.deserializeBinary(new Uint8Array(buffer_arg));
}


var RemoteService = exports.RemoteService = {
  // receiver register
register: {
    path: '/remote.Remote/Register',
    requestStream: false,
    responseStream: false,
    requestType: remote_pb.RegisterRequest,
    responseType: remote_pb.RegisterResponse,
    requestSerialize: serialize_remote_RegisterRequest,
    requestDeserialize: deserialize_remote_RegisterRequest,
    responseSerialize: serialize_remote_RegisterResponse,
    responseDeserialize: deserialize_remote_RegisterResponse,
  },
  getReceivers: {
    path: '/remote.Remote/GetReceivers',
    requestStream: false,
    responseStream: false,
    requestType: remote_pb.GetReceiversRequest,
    responseType: remote_pb.GetReceiversResponse,
    requestSerialize: serialize_remote_GetReceiversRequest,
    requestDeserialize: deserialize_remote_GetReceiversRequest,
    responseSerialize: serialize_remote_GetReceiversResponse,
    responseDeserialize: deserialize_remote_GetReceiversResponse,
  },
  send: {
    path: '/remote.Remote/Send',
    requestStream: true,
    responseStream: false,
    requestType: remote_pb.SendRequest,
    responseType: remote_pb.EmptyResponse,
    requestSerialize: serialize_remote_SendRequest,
    requestDeserialize: deserialize_remote_SendRequest,
    responseSerialize: serialize_remote_EmptyResponse,
    responseDeserialize: deserialize_remote_EmptyResponse,
  },
  receive: {
    path: '/remote.Remote/Receive',
    requestStream: false,
    responseStream: true,
    requestType: remote_pb.ReceiveRequest,
    responseType: remote_pb.ReceiveResponse,
    requestSerialize: serialize_remote_ReceiveRequest,
    requestDeserialize: deserialize_remote_ReceiveRequest,
    responseSerialize: serialize_remote_ReceiveResponse,
    responseDeserialize: deserialize_remote_ReceiveResponse,
  },
};

exports.RemoteClient = grpc.makeGenericClientConstructor(RemoteService);
