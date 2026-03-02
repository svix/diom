// this file is @generated





export interface MsgStreamReceiveIn {
    topic: string;
consumerGroup: string;
batchSize?: number;
leaseDurationMillis?: number;
}

export const MsgStreamReceiveInSerializer = {
    _fromJsonObject(object: any): MsgStreamReceiveIn {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            batchSize: object['batch_size'],
            leaseDurationMillis: object['lease_duration_millis'],
            };
    },

    _toJsonObject(self: MsgStreamReceiveIn): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'batch_size': self.batchSize,
            'lease_duration_millis': self.leaseDurationMillis,
            };
    }
}