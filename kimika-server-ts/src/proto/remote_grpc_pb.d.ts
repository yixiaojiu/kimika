// package: remote
// file: remote.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as remote_pb from "./remote_pb";

interface IRemoteService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    register: IRemoteService_IRegister;
    getReceivers: IRemoteService_IGetReceivers;
    send: IRemoteService_ISend;
    receive: IRemoteService_IReceive;
}

interface IRemoteService_IRegister extends grpc.MethodDefinition<remote_pb.RegisterRequest, remote_pb.RegisterResponse> {
    path: "/remote.Remote/Register";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<remote_pb.RegisterRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.RegisterRequest>;
    responseSerialize: grpc.serialize<remote_pb.RegisterResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.RegisterResponse>;
}
interface IRemoteService_IGetReceivers extends grpc.MethodDefinition<remote_pb.GetReceiversRequest, remote_pb.GetReceiversResponse> {
    path: "/remote.Remote/GetReceivers";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<remote_pb.GetReceiversRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.GetReceiversRequest>;
    responseSerialize: grpc.serialize<remote_pb.GetReceiversResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.GetReceiversResponse>;
}
interface IRemoteService_ISend extends grpc.MethodDefinition<remote_pb.SendRequest, remote_pb.EmptyResponse> {
    path: "/remote.Remote/Send";
    requestStream: true;
    responseStream: false;
    requestSerialize: grpc.serialize<remote_pb.SendRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.SendRequest>;
    responseSerialize: grpc.serialize<remote_pb.EmptyResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.EmptyResponse>;
}
interface IRemoteService_IReceive extends grpc.MethodDefinition<remote_pb.ReceiveRequest, remote_pb.ReceiveResponse> {
    path: "/remote.Remote/Receive";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<remote_pb.ReceiveRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.ReceiveRequest>;
    responseSerialize: grpc.serialize<remote_pb.ReceiveResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.ReceiveResponse>;
}

export const RemoteService: IRemoteService;

export interface IRemoteServer extends grpc.UntypedServiceImplementation {
    register: grpc.handleUnaryCall<remote_pb.RegisterRequest, remote_pb.RegisterResponse>;
    getReceivers: grpc.handleServerStreamingCall<remote_pb.GetReceiversRequest, remote_pb.GetReceiversResponse>;
    send: grpc.handleClientStreamingCall<remote_pb.SendRequest, remote_pb.EmptyResponse>;
    receive: grpc.handleServerStreamingCall<remote_pb.ReceiveRequest, remote_pb.ReceiveResponse>;
}

export interface IRemoteClient {
    register(request: remote_pb.RegisterRequest, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    register(request: remote_pb.RegisterRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    register(request: remote_pb.RegisterRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    getReceivers(request: remote_pb.GetReceiversRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    getReceivers(request: remote_pb.GetReceiversRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    send(callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    send(metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    send(options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    send(metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    receive(request: remote_pb.ReceiveRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ReceiveResponse>;
    receive(request: remote_pb.ReceiveRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ReceiveResponse>;
}

export class RemoteClient extends grpc.Client implements IRemoteClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public register(request: remote_pb.RegisterRequest, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    public register(request: remote_pb.RegisterRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    public register(request: remote_pb.RegisterRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterResponse) => void): grpc.ClientUnaryCall;
    public getReceivers(request: remote_pb.GetReceiversRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    public getReceivers(request: remote_pb.GetReceiversRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    public send(callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    public send(metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    public send(options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    public send(metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.SendRequest>;
    public receive(request: remote_pb.ReceiveRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ReceiveResponse>;
    public receive(request: remote_pb.ReceiveRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ReceiveResponse>;
}
