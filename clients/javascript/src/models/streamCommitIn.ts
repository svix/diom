// this file is @generated





export interface StreamCommitIn {
    consumerGroup: string;
name: string;
offset: number;
topic: string;
}

export const StreamCommitInSerializer = {
    _fromJsonObject(object: any): StreamCommitIn {
        return {
            consumerGroup: object['consumer_group'],
            name: object['name'],
            offset: object['offset'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: StreamCommitIn): any {
        return {
            'consumer_group': self.consumerGroup,
            'name': self.name,
            'offset': self.offset,
            'topic': self.topic,
            };
    }
}