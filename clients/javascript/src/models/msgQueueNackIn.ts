// this file is @generated

export interface MsgQueueNackIn {
    topic: string;
    consumerGroup: string;
    msgIds: string[];
}

export const MsgQueueNackInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueNackIn {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            msgIds: object['msg_ids'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueNackIn): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'msg_ids': self.msgIds,
        };
    }
}