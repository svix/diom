// this file is @generated





export interface StreamReceiveIn {
    batchSize?: number;
consumerGroup: string;
leaseDurationMillis?: number;
name: string;
topic: string;
}

export const StreamReceiveInSerializer = {
    _fromJsonObject(object: any): StreamReceiveIn {
        return {
            batchSize: object['batch_size'],
            consumerGroup: object['consumer_group'],
            leaseDurationMillis: object['lease_duration_millis'],
            name: object['name'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: StreamReceiveIn): any {
        return {
            'batch_size': self.batchSize,
            'consumer_group': self.consumerGroup,
            'lease_duration_millis': self.leaseDurationMillis,
            'name': self.name,
            'topic': self.topic,
            };
    }
}