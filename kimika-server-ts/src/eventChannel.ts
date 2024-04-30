interface Message {
	eventName: string
	value?: any
}

export class EventChannel {
	private eventMap = new Map<string, ((value?: any) => void)[]>()

	private messageQueue: Message[] = []

	on<T = any>(eventName: string): Promise<T> {
		const index = this.messageQueue.findLastIndex(
			(m) => m.eventName === eventName,
		)
		if (index >= 0) {
			const message = this.messageQueue[index]
			this.messageQueue.splice(index, 1)
			return Promise.resolve(message.value)
		}

		return new Promise<T>((resolve) => {
			const fns = this.eventMap.get(eventName)
			if (fns) {
				fns.push(resolve)
			} else {
				this.eventMap.set(eventName, [resolve])
			}
		})
	}

	emit(eventName: string, value?: any) {
		const fns = this.eventMap.get(eventName)
		if (fns?.length) {
			for (const fn of fns) {
				fn(value)
			}
			this.eventMap.delete(eventName)
		} else {
			this.messageQueue.push({ eventName, value })
		}
	}
}
