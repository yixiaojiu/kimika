import remote_pb from './proto/remote_pb';
import { info } from './logger';

interface Receiver {
  // register time
  timestamp: number;
  alias: string;
  ip: string;
  id: string;
}

interface Content {
  // register time
  timestamp: number;
  contentType: remote_pb.Type;
  alias: string;
  ip: string;
  senderId: string;
  contentId: string;
  size?: number;
  name?: string;
}

export const receiverMap = new Map<string, Receiver>();

export const contentMap = new Map<string, Content>();

// 1 hour
const EXPIRES = 60 * 60 * 1000;

export function checkState(target: 'receive' | 'content') {
  const map = target === 'receive' ? receiverMap : contentMap;

  const removeKeys: string[] = [];
  for (const [key, value] of map) {
    if (Date.now() - value.timestamp > EXPIRES) {
      removeKeys.push(key);
      map.delete(key);
    }
  }

  removeKeys.length && info(`[checkState] ${target} remove keys: ${removeKeys.join(',')}`);
}
