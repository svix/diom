// this file is @generated





export interface StreamCommitIn {
    consumerGroup: string;
offset: number;
topic: string;
}

export const StreamCommitInSerializer = {
    _fromJsonObject(object: any): StreamCommitIn {
        return {
            consumerGroup: object['consumer_group'],
            offset: object['offset'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: StreamCommitIn): any {
        return {
            'consumer_group': self.consumerGroup,
            'offset': self.offset,
            'topic': self.topic,
            };
    }
}