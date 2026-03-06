// this file is @generated

export interface MsgStreamCommitIn {
    offset: number;
}

export interface MsgStreamCommitIn_ {
    topic: string;
    consumerGroup: string;
    offset: number;
}

export const MsgStreamCommitInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamCommitIn_ {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            offset: object['offset'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamCommitIn_): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'offset': self.offset,
        };
    }
}