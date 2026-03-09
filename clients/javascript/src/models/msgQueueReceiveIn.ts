// this file is @generated

export interface MsgQueueReceiveIn {
    topic: string;
    consumerGroup: string;
    batchSize?: number;
    leaseDurationMillis?: number;
}

export const MsgQueueReceiveInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueReceiveIn {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            batchSize: object['batch_size'],
            leaseDurationMillis: object['lease_duration_millis'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueReceiveIn): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'batch_size': self.batchSize,
            'lease_duration_millis': self.leaseDurationMillis,
        };
    }
}