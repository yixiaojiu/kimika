import * as grpc from "@grpc/grpc-js"
import { info } from "./logger"
import type { ServerMethodDefinition } from "@grpc/grpc-js/build/src/make-client"

const filterMethod = ["Receive", "Send"]

export function loggingInterceptor(
	methodDescriptor: ServerMethodDefinition<any, any>,
	call: grpc.ServerInterceptingCallInterface,
) {
	const method = methodDescriptor.path.split("/").pop()!
	const ip = call.getPeer().split(":")[0]
	const isFilterMethod = filterMethod.includes(method)
	if (isFilterMethod) {
		info(`[${method}] ${ip}`)
	}
	const listener = new grpc.ServerListenerBuilder()
		.withOnReceiveMessage((message, next) => {
			if (!isFilterMethod) {
				info(`ReceiveMessage [${method}] ${ip} - ${message.array}`)
			}
			next(message)
		})
		.build()
	const responder = new grpc.ResponderBuilder()
		.withStart((next) => {
			next(listener)
		})
		.withSendMessage((message, next) => {
			if (!isFilterMethod) {
				info(`SendMessage [${method}] ${ip} - ${message.array}`)
			}
			next(message)
		})
		.build()
	return new grpc.ServerInterceptingCall(call, responder)
}
