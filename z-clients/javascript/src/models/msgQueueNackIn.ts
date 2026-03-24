// this file is @generated

export interface MsgQueueNackIn {
    namespace?: string | null;
    msgIds: string[];
}

export interface MsgQueueNackIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    msgIds: string[];
}

export const MsgQueueNackInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueNackIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            msgIds: object['msg_ids'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueNackIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'msg_ids': self.msgIds,
        };
    }
}