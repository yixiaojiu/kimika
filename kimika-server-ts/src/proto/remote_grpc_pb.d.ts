// package: remote
// file: remote.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as remote_pb from "./remote_pb";

interface IRemoteService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    registerReceiver: IRemoteService_IRegisterReceiver;
    registerContent: IRemoteService_IRegisterContent;
    getContent: IRemoteService_IGetContent;
    getReceivers: IRemoteService_IGetReceivers;
    chooseReceiver: IRemoteService_IChooseReceiver;
    send: IRemoteService_ISend;
    receive: IRemoteService_IReceive;
}

interface IRemoteService_IRegisterReceiver extends grpc.MethodDefinition<remote_pb.RegisterReceiverRequest, remote_pb.RegisterReceiverResponse> {
    path: "/remote.Remote/RegisterReceiver";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<remote_pb.RegisterReceiverRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.RegisterReceiverRequest>;
    responseSerialize: grpc.serialize<remote_pb.RegisterReceiverResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.RegisterReceiverResponse>;
}
interface IRemoteService_IRegisterContent extends grpc.MethodDefinition<remote_pb.RegisterContentRequest, remote_pb.RegisterContentResponse> {
    path: "/remote.Remote/RegisterContent";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<remote_pb.RegisterContentRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.RegisterContentRequest>;
    responseSerialize: grpc.serialize<remote_pb.RegisterContentResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.RegisterContentResponse>;
}
interface IRemoteService_IGetContent extends grpc.MethodDefinition<remote_pb.GetContentRequest, remote_pb.GetContentReponse> {
    path: "/remote.Remote/GetContent";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<remote_pb.GetContentRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.GetContentRequest>;
    responseSerialize: grpc.serialize<remote_pb.GetContentReponse>;
    responseDeserialize: grpc.deserialize<remote_pb.GetContentReponse>;
}
interface IRemoteService_IGetReceivers extends grpc.MethodDefinition<remote_pb.EmptyRequest, remote_pb.GetReceiversResponse> {
    path: "/remote.Remote/GetReceivers";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<remote_pb.EmptyRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.EmptyRequest>;
    responseSerialize: grpc.serialize<remote_pb.GetReceiversResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.GetReceiversResponse>;
}
interface IRemoteService_IChooseReceiver extends grpc.MethodDefinition<remote_pb.ChooseReceiverRequest, remote_pb.ChooseReceiverResponse> {
    path: "/remote.Remote/ChooseReceiver";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<remote_pb.ChooseReceiverRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.ChooseReceiverRequest>;
    responseSerialize: grpc.serialize<remote_pb.ChooseReceiverResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.ChooseReceiverResponse>;
}
interface IRemoteService_ISend extends grpc.MethodDefinition<remote_pb.TransferContent, remote_pb.EmptyResponse> {
    path: "/remote.Remote/Send";
    requestStream: true;
    responseStream: false;
    requestSerialize: grpc.serialize<remote_pb.TransferContent>;
    requestDeserialize: grpc.deserialize<remote_pb.TransferContent>;
    responseSerialize: grpc.serialize<remote_pb.EmptyResponse>;
    responseDeserialize: grpc.deserialize<remote_pb.EmptyResponse>;
}
interface IRemoteService_IReceive extends grpc.MethodDefinition<remote_pb.ReceiveRequest, remote_pb.TransferContent> {
    path: "/remote.Remote/Receive";
    requestStream: false;
    responseStream: true;
    requestSerialize: grpc.serialize<remote_pb.ReceiveRequest>;
    requestDeserialize: grpc.deserialize<remote_pb.ReceiveRequest>;
    responseSerialize: grpc.serialize<remote_pb.TransferContent>;
    responseDeserialize: grpc.deserialize<remote_pb.TransferContent>;
}

export const RemoteService: IRemoteService;

export interface IRemoteServer extends grpc.UntypedServiceImplementation {
    registerReceiver: grpc.handleUnaryCall<remote_pb.RegisterReceiverRequest, remote_pb.RegisterReceiverResponse>;
    registerContent: grpc.handleUnaryCall<remote_pb.RegisterContentRequest, remote_pb.RegisterContentResponse>;
    getContent: grpc.handleServerStreamingCall<remote_pb.GetContentRequest, remote_pb.GetContentReponse>;
    getReceivers: grpc.handleServerStreamingCall<remote_pb.EmptyRequest, remote_pb.GetReceiversResponse>;
    chooseReceiver: grpc.handleServerStreamingCall<remote_pb.ChooseReceiverRequest, remote_pb.ChooseReceiverResponse>;
    send: grpc.handleClientStreamingCall<remote_pb.TransferContent, remote_pb.EmptyResponse>;
    receive: grpc.handleServerStreamingCall<remote_pb.ReceiveRequest, remote_pb.TransferContent>;
}

export interface IRemoteClient {
    registerReceiver(request: remote_pb.RegisterReceiverRequest, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterReceiverResponse) => void): grpc.ClientUnaryCall;
    registerReceiver(request: remote_pb.RegisterReceiverRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterReceiverResponse) => void): grpc.ClientUnaryCall;
    registerReceiver(request: remote_pb.RegisterReceiverRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterReceiverResponse) => void): grpc.ClientUnaryCall;
    registerContent(request: remote_pb.RegisterContentRequest, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterContentResponse) => void): grpc.ClientUnaryCall;
    registerContent(request: remote_pb.RegisterContentRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterContentResponse) => void): grpc.ClientUnaryCall;
    registerContent(request: remote_pb.RegisterContentRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterContentResponse) => void): grpc.ClientUnaryCall;
    getContent(request: remote_pb.GetContentRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetContentReponse>;
    getContent(request: remote_pb.GetContentRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetContentReponse>;
    getReceivers(request: remote_pb.EmptyRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    getReceivers(request: remote_pb.EmptyRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    chooseReceiver(request: remote_pb.ChooseReceiverRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ChooseReceiverResponse>;
    chooseReceiver(request: remote_pb.ChooseReceiverRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ChooseReceiverResponse>;
    send(callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    send(metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    send(options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    send(metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    receive(request: remote_pb.ReceiveRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.TransferContent>;
    receive(request: remote_pb.ReceiveRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.TransferContent>;
}

export class RemoteClient extends grpc.Client implements IRemoteClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public registerReceiver(request: remote_pb.RegisterReceiverRequest, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterReceiverResponse) => void): grpc.ClientUnaryCall;
    public registerReceiver(request: remote_pb.RegisterReceiverRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterReceiverResponse) => void): grpc.ClientUnaryCall;
    public registerReceiver(request: remote_pb.RegisterReceiverRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterReceiverResponse) => void): grpc.ClientUnaryCall;
    public registerContent(request: remote_pb.RegisterContentRequest, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterContentResponse) => void): grpc.ClientUnaryCall;
    public registerContent(request: remote_pb.RegisterContentRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterContentResponse) => void): grpc.ClientUnaryCall;
    public registerContent(request: remote_pb.RegisterContentRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.RegisterContentResponse) => void): grpc.ClientUnaryCall;
    public getContent(request: remote_pb.GetContentRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetContentReponse>;
    public getContent(request: remote_pb.GetContentRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetContentReponse>;
    public getReceivers(request: remote_pb.EmptyRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    public getReceivers(request: remote_pb.EmptyRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.GetReceiversResponse>;
    public chooseReceiver(request: remote_pb.ChooseReceiverRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ChooseReceiverResponse>;
    public chooseReceiver(request: remote_pb.ChooseReceiverRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.ChooseReceiverResponse>;
    public send(callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    public send(metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    public send(options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    public send(metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: remote_pb.EmptyResponse) => void): grpc.ClientWritableStream<remote_pb.TransferContent>;
    public receive(request: remote_pb.ReceiveRequest, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.TransferContent>;
    public receive(request: remote_pb.ReceiveRequest, metadata?: grpc.Metadata, options?: Partial<grpc.CallOptions>): grpc.ClientReadableStream<remote_pb.TransferContent>;
}
