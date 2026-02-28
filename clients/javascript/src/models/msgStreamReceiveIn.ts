// this file is @generated





export interface MsgStreamReceiveIn {
    batchSize?: number;
consumerGroup: string;
leaseDurationMillis?: number;
topic: string;
}

export const MsgStreamReceiveInSerializer = {
    _fromJsonObject(object: any): MsgStreamReceiveIn {
        return {
            batchSize: object['batch_size'],
            consumerGroup: object['consumer_group'],
            leaseDurationMillis: object['lease_duration_millis'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: MsgStreamReceiveIn): any {
        return {
            'batch_size': self.batchSize,
            'consumer_group': self.consumerGroup,
            'lease_duration_millis': self.leaseDurationMillis,
            'topic': self.topic,
            };
    }
}