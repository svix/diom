// this file is @generated





export interface MsgStreamCommitIn {
    consumerGroup: string;
offset: number;
topic: string;
}

export const MsgStreamCommitInSerializer = {
    _fromJsonObject(object: any): MsgStreamCommitIn {
        return {
            consumerGroup: object['consumer_group'],
            offset: object['offset'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: MsgStreamCommitIn): any {
        return {
            'consumer_group': self.consumerGroup,
            'offset': self.offset,
            'topic': self.topic,
            };
    }
}