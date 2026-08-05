// this file is @generated

export interface MsgFifoAckIn {
    namespace?: string | null;
    msgIds: string[];
}

export interface MsgFifoAckIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    msgIds: string[];
}

export const MsgFifoAckInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgFifoAckIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            msgIds: object['msg_ids'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgFifoAckIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'msg_ids': self.msgIds,
        };
    }
}