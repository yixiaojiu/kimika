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

export class RegisterReceiverResponse extends jspb.Message { 
    getReceiverId(): string;
    setReceiverId(value: string): RegisterReceiverResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterReceiverResponse.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterReceiverResponse): RegisterReceiverResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterReceiverResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterReceiverResponse;
    static deserializeBinaryFromReader(message: RegisterReceiverResponse, reader: jspb.BinaryReader): RegisterReceiverResponse;
}

export namespace RegisterReceiverResponse {
    export type AsObject = {
        receiverId: string,
    }
}

export class RegisterContentResponse extends jspb.Message { 
    getContentId(): string;
    setContentId(value: string): RegisterContentResponse;
    getSenderId(): string;
    setSenderId(value: string): RegisterContentResponse;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RegisterContentResponse.AsObject;
    static toObject(includeInstance: boolean, msg: RegisterContentResponse): RegisterContentResponse.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RegisterContentResponse, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RegisterContentResponse;
    static deserializeBinaryFromReader(message: RegisterContentResponse, reader: jspb.BinaryReader): RegisterContentResponse;
}

export namespace RegisterContentResponse {
    export type AsObject = {
        contentId: string,
        senderId: string,
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

export class GetContentRequest extends jspb.Message { 
    getReceiverId(): string;
    setReceiverId(value: string): GetContentRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): GetContentRequest.AsObject;
    static toObject(includeInstance: boolean, msg: GetContentRequest): GetContentRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: GetContentRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): GetContentRequest;
    static deserializeBinaryFromReader(message: GetContentRequest, reader: jspb.BinaryReader): GetContentRequest;
}

export namespace GetContentRequest {
    export type AsObject = {
        receiverId: string,
    }
}

export class GetContentReponse extends jspb.Message { 
    clearContentListList(): void;
    getContentListList(): Array<GetContentReponse.Content>;
    setContentListList(value: Array<GetContentReponse.Content>): GetContentReponse;
    addContentList(value?: GetContentReponse.Content, index?: number): GetContentReponse.Content;

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
        contentListList: Array<GetContentReponse.Content.AsObject>,
    }


    export class Content extends jspb.Message { 
        getContentType(): Type;
        setContentType(value: Type): Content;
        getAlias(): string;
        setAlias(value: string): Content;
        getIp(): string;
        setIp(value: string): Content;
        getSenderId(): string;
        setSenderId(value: string): Content;
        getContentId(): string;
        setContentId(value: string): Content;

        hasSize(): boolean;
        clearSize(): void;
        getSize(): number | undefined;
        setSize(value: number): Content;

        hasName(): boolean;
        clearName(): void;
        getName(): string | undefined;
        setName(value: string): Content;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Content.AsObject;
        static toObject(includeInstance: boolean, msg: Content): Content.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: Content, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Content;
        static deserializeBinaryFromReader(message: Content, reader: jspb.BinaryReader): Content;
    }

    export namespace Content {
        export type AsObject = {
            contentType: Type,
            alias: string,
            ip: string,
            senderId: string,
            contentId: string,
            size?: number,
            name?: string,
        }
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
        getReceiverId(): string;
        setReceiverId(value: string): Receiver;

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
            receiverId: string,
        }
    }

}

export class ChooseReceiverRequest extends jspb.Message { 
    getReceiverId(): string;
    setReceiverId(value: string): ChooseReceiverRequest;
    getSenderId(): string;
    setSenderId(value: string): ChooseReceiverRequest;

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
        receiverId: string,
        senderId: string,
    }
}

export class ChooseReceiverResponse extends jspb.Message { 
    getContentId(): string;
    setContentId(value: string): ChooseReceiverResponse;
    getReceiverId(): string;
    setReceiverId(value: string): ChooseReceiverResponse;

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
        contentId: string,
        receiverId: string,
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
    getReceiverId(): string;
    setReceiverId(value: string): ReceiveRequest;
    getContentId(): string;
    setContentId(value: string): ReceiveRequest;

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
        receiverId: string,
        contentId: string,
    }
}

export enum Type {
    FILE = 0,
    MESSAGE = 1,
}
