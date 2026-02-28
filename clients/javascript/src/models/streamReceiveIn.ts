// this file is @generated





export interface StreamReceiveIn {
    batchSize?: number;
consumerGroup: string;
leaseDurationMillis?: number;
topic: string;
}

export const StreamReceiveInSerializer = {
    _fromJsonObject(object: any): StreamReceiveIn {
        return {
            batchSize: object['batch_size'],
            consumerGroup: object['consumer_group'],
            leaseDurationMillis: object['lease_duration_millis'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: StreamReceiveIn): any {
        return {
            'batch_size': self.batchSize,
            'consumer_group': self.consumerGroup,
            'lease_duration_millis': self.leaseDurationMillis,
            'topic': self.topic,
            };
    }
}