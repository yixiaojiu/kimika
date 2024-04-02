import { EventEmitter } from 'events';

export const receiverEmitter = new EventEmitter();

export const contentEmitter = new EventEmitter();

export function onReceiver() {
  return new Promise<void>(resolve => {
    receiverEmitter.on('new', resolve);
  });
}

export function emitReceiver() {
  receiverEmitter.emit('new');
}

export function onContent() {
  return new Promise<void>(resolve => {
    contentEmitter.on('new', resolve);
  });
}

export function emitContent() {
  contentEmitter.emit('new');
}
