import { EventEmitter } from "node:events"
import type { Duplex, Writable } from "node:stream"
import { EventChannel } from "./eventChannel"

export const newReceiverEmitter = new EventEmitter()

export const contentChannel = new EventChannel()

export const senderEmitter = new EventEmitter()

export const streamEmitter = new EventEmitter()

export function onReceiver() {
	return new Promise<void>((resolve) => {
		newReceiverEmitter.once("new", resolve)
	})
}

export function emitReceiver() {
	newReceiverEmitter.emit("new")
}

export function onSender(receiverId: string) {
	return new Promise<string>((resolve) => {
		senderEmitter.once(receiverId, (contentId) => {
			resolve(contentId)
		})
	})
}

export function emitSender(receiverId: string, contentId: string) {
	senderEmitter.emit(receiverId, contentId)
}

export function onStream(contentId: string) {
	return new Promise<Writable>((resolve) => {
		streamEmitter.once(contentId, (stream) => {
			resolve(stream)
		})
	})
}

export function emitStream(contentId: string, stream: Duplex) {
	streamEmitter.emit(contentId, stream)
}
