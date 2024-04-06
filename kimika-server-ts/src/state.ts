import remote_pb from './proto/remote_pb';

interface Receiver {
  alias: string;
  ip: string;
  id: string;
}

interface Content {
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
