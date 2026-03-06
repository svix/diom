// this file is @generated

export interface MsgStreamCommitIn {
    topic: string;
    consumerGroup: string;
    offset: number;
}

export const MsgStreamCommitInSerializer = {
    _fromJsonObject(object: any): MsgStreamCommitIn {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            offset: object['offset'],
            };
    },

    _toJsonObject(self: MsgStreamCommitIn): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'offset': self.offset,
            };
    }
}