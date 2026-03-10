// this file is @generated

export interface MsgQueueRedriveDlqIn {
    topic: string;
    consumerGroup: string;
}

export const MsgQueueRedriveDlqInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueRedriveDlqIn {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueRedriveDlqIn): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
        };
    }
}