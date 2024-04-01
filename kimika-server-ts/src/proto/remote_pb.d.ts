// package: remote
// file: remote.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class EmptyRequest extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EmptyRequest.AsObject;
    static toObject(includeInstance: boolean, msg: EmptyRequest): EmptyRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EmptyRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EmptyRequest;
    static deserializeBinaryFromReader(message: EmptyRequest, reader: jspb.BinaryReader): EmptyRequest;
}

export namespace EmptyRequest {
    export type AsObject = {
    }
}

export class EmptyResponse extends jspb.Message { 

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EmptyResponse.AsObject;
    static toObject(includeInstance: boolean, msg: EmptyResponse): EmptyResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: EmptyResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EmptyResponse;
    static deserializeBinaryFromReader(message: EmptyResponse, reader: jspb.BinaryReader): EmptyResponse;
}

export namespace EmptyResponse {
    export type AsObject = {
    }
}

export class RegisterRequest extends jspb.Message { 
    getAlias(): string;
    setAlias(value: string): RegisterRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterRequest): RegisterRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterRequest;
    static deserializeBinaryFromReader(message: RegisterRequest, reader: jspb.BinaryReader): RegisterRequest;
}

export namespace RegisterRequest {
    export type AsObject = {
        alias: string,
    }
}

export class RegisterResponse extends jspb.Message { 
    getContentType(): RegisterResponse.Type;
    setContentType(value: RegisterResponse.Type): RegisterResponse;
    getId(): string;
    setId(value: string): RegisterResponse;

    hasSize(): boolean;
    clearSize(): void;
    getSize(): number | undefined;
    setSize(value: number): RegisterResponse;

    hasName(): boolean;
    clearName(): void;
    getName(): string | undefined;
    setName(value: string): RegisterResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterResponse.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterResponse): RegisterResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterResponse;
    static deserializeBinaryFromReader(message: RegisterResponse, reader: jspb.BinaryReader): RegisterResponse;
}

export namespace RegisterResponse {
    export type AsObject = {
        contentType: RegisterResponse.Type,
        id: string,
        size?: number,
        name?: string,
    }

    export enum Type {
    FILE = 0,
    MESSAGE = 1,
    }

}

export class GetReceiversRequest extends jspb.Message { 

    hasAlias(): boolean;
    clearAlias(): void;
    getAlias(): string | undefined;
    setAlias(value: string): GetReceiversRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetReceiversRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetReceiversRequest): GetReceiversRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetReceiversRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetReceiversRequest;
    static deserializeBinaryFromReader(message: GetReceiversRequest, reader: jspb.BinaryReader): GetReceiversRequest;
}

export namespace GetReceiversRequest {
    export type AsObject = {
        alias?: string,
    }
}

export class GetReceiversResponse extends jspb.Message { 
    clearReceiversList(): void;
    getReceiversList(): Array<GetReceiversResponse.Receiver>;
    setReceiversList(value: Array<GetReceiversResponse.Receiver>): GetReceiversResponse;
    addReceivers(value?: GetReceiversResponse.Receiver, index?: number): GetReceiversResponse.Receiver;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetReceiversResponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetReceiversResponse): GetReceiversResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetReceiversResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetReceiversResponse;
    static deserializeBinaryFromReader(message: GetReceiversResponse, reader: jspb.BinaryReader): GetReceiversResponse;
}

export namespace GetReceiversResponse {
    export type AsObject = {
        receiversList: Array<GetReceiversResponse.Receiver.AsObject>,
    }


    export class Receiver extends jspb.Message { 
        getAlias(): string;
        setAlias(value: string): Receiver;
        getId(): string;
        setId(value: string): Receiver;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Receiver.AsObject;
        static toObject(includeInstance: boolean, msg: Receiver): Receiver.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: Receiver, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Receiver;
        static deserializeBinaryFromReader(message: Receiver, reader: jspb.BinaryReader): Receiver;
    }

    export namespace Receiver {
        export type AsObject = {
            alias: string,
            id: string,
        }
    }

}

export class SendRequest extends jspb.Message { 
    getData(): Uint8Array | string;
    getData_asU8(): Uint8Array;
    getData_asB64(): string;
    setData(value: Uint8Array | string): SendRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): SendRequest.AsObject;
    static toObject(includeInstance: boolean, msg: SendRequest): SendRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: SendRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): SendRequest;
    static deserializeBinaryFromReader(message: SendRequest, reader: jspb.BinaryReader): SendRequest;
}

export namespace SendRequest {
    export type AsObject = {
        data: Uint8Array | string,
    }
}

export class ReceiveRequest extends jspb.Message { 
    getId(): string;
    setId(value: string): ReceiveRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ReceiveRequest.AsObject;
    static toObject(includeInstance: boolean, msg: ReceiveRequest): ReceiveRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ReceiveRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ReceiveRequest;
    static deserializeBinaryFromReader(message: ReceiveRequest, reader: jspb.BinaryReader): ReceiveRequest;
}

export namespace ReceiveRequest {
    export type AsObject = {
        id: string,
    }
}

export class ReceiveResponse extends jspb.Message { 
    getData(): Uint8Array | string;
    getData_asU8(): Uint8Array;
    getData_asB64(): string;
    setData(value: Uint8Array | string): ReceiveResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ReceiveResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ReceiveResponse): ReceiveResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ReceiveResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ReceiveResponse;
    static deserializeBinaryFromReader(message: ReceiveResponse, reader: jspb.BinaryReader): ReceiveResponse;
}

export namespace ReceiveResponse {
    export type AsObject = {
        data: Uint8Array | string,
    }
}
