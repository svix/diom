// this file is @generated

export interface SinkConfigureOut {
    topic: string;
    consumerGroup: string;
}

export const SinkConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SinkConfigureOut {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SinkConfigureOut): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
        };
    }
}