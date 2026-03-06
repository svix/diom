// this file is @generated
// biome-ignore-all lint/suspicious/noEmptyInterface: forwards compat

export interface MsgQueueRedriveDlqIn {
}

export interface MsgQueueRedriveDlqIn_ {
    topic: string;
    consumerGroup: string;
}

export const MsgQueueRedriveDlqInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueRedriveDlqIn_ {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueRedriveDlqIn_): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
        };
    }
}