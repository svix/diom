// this file is @generated

export interface MsgQueueNackIn {
    msgIds: string[];
}

export interface MsgQueueNackIn_ {
    topic: string;
    consumerGroup: string;
    msgIds: string[];
}

export const MsgQueueNackInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueNackIn_ {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            msgIds: object['msg_ids'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueNackIn_): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'msg_ids': self.msgIds,
        };
    }
}