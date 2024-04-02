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

export class RegisterReceiverRequest extends jspb.Message { 
    getAlias(): string;
    setAlias(value: string): RegisterReceiverRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterReceiverRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterReceiverRequest): RegisterReceiverRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterReceiverRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterReceiverRequest;
    static deserializeBinaryFromReader(message: RegisterReceiverRequest, reader: jspb.BinaryReader): RegisterReceiverRequest;
}

export namespace RegisterReceiverRequest {
    export type AsObject = {
        alias: string,
    }
}

export class RegisterResponse extends jspb.Message { 
    getId(): string;
    setId(value: string): RegisterResponse;

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
        id: string,
    }
}

export class RegisterContentRequest extends jspb.Message { 
    getContentType(): Type;
    setContentType(value: Type): RegisterContentRequest;
    getAlias(): string;
    setAlias(value: string): RegisterContentRequest;

    hasSize(): boolean;
    clearSize(): void;
    getSize(): number | undefined;
    setSize(value: number): RegisterContentRequest;

    hasName(): boolean;
    clearName(): void;
    getName(): string | undefined;
    setName(value: string): RegisterContentRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterContentRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterContentRequest): RegisterContentRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterContentRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterContentRequest;
    static deserializeBinaryFromReader(message: RegisterContentRequest, reader: jspb.BinaryReader): RegisterContentRequest;
}

export namespace RegisterContentRequest {
    export type AsObject = {
        contentType: Type,
        alias: string,
        size?: number,
        name?: string,
    }
}

export class GetContentReponse extends jspb.Message { 
    getContentType(): Type;
    setContentType(value: Type): GetContentReponse;
    getAlias(): string;
    setAlias(value: string): GetContentReponse;
    getIp(): string;
    setIp(value: string): GetContentReponse;

    hasSize(): boolean;
    clearSize(): void;
    getSize(): number | undefined;
    setSize(value: number): GetContentReponse;

    hasName(): boolean;
    clearName(): void;
    getName(): string | undefined;
    setName(value: string): GetContentReponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetContentReponse.AsObject;
    static toObject(includeInstance: boolean, msg: GetContentReponse): GetContentReponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetContentReponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetContentReponse;
    static deserializeBinaryFromReader(message: GetContentReponse, reader: jspb.BinaryReader): GetContentReponse;
}

export namespace GetContentReponse {
    export type AsObject = {
        contentType: Type,
        alias: string,
        ip: string,
        size?: number,
        name?: string,
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
        getIp(): string;
        setIp(value: string): Receiver;

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
            ip: string,
        }
    }

}

export class ChooseReceiverRequest extends jspb.Message { 
    getId(): string;
    setId(value: string): ChooseReceiverRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ChooseReceiverRequest.AsObject;
    static toObject(includeInstance: boolean, msg: ChooseReceiverRequest): ChooseReceiverRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ChooseReceiverRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ChooseReceiverRequest;
    static deserializeBinaryFromReader(message: ChooseReceiverRequest, reader: jspb.BinaryReader): ChooseReceiverRequest;
}

export namespace ChooseReceiverRequest {
    export type AsObject = {
        id: string,
    }
}

export class ChooseReceiverResponse extends jspb.Message { 
    getId(): string;
    setId(value: string): ChooseReceiverResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ChooseReceiverResponse.AsObject;
    static toObject(includeInstance: boolean, msg: ChooseReceiverResponse): ChooseReceiverResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ChooseReceiverResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ChooseReceiverResponse;
    static deserializeBinaryFromReader(message: ChooseReceiverResponse, reader: jspb.BinaryReader): ChooseReceiverResponse;
}

export namespace ChooseReceiverResponse {
    export type AsObject = {
        id: string,
    }
}

export class TransferContent extends jspb.Message { 
    getData(): Uint8Array | string;
    getData_asU8(): Uint8Array;
    getData_asB64(): string;
    setData(value: Uint8Array | string): TransferContent;
    clearRangeList(): void;
    getRangeList(): Array<number>;
    setRangeList(value: Array<number>): TransferContent;
    addRange(value: number, index?: number): number;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): TransferContent.AsObject;
    static toObject(includeInstance: boolean, msg: TransferContent): TransferContent.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: TransferContent, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): TransferContent;
    static deserializeBinaryFromReader(message: TransferContent, reader: jspb.BinaryReader): TransferContent;
}

export namespace TransferContent {
    export type AsObject = {
        data: Uint8Array | string,
        rangeList: Array<number>,
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

export enum Type {
    FILE = 0,
    MESSAGE = 1,
}
