// this file is @generated

export interface SinkDeleteOut {
    topic: string;
    consumerGroup: string;
    success: boolean;
}

export const SinkDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SinkDeleteOut {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SinkDeleteOut): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'success': self.success,
        };
    }
}