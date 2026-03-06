// this file is @generated

export interface MsgQueueReceiveIn {
    topic: string;
    batchSize?: number;
    leaseDurationMillis?: number;
}

export const MsgQueueReceiveInSerializer = {
    _fromJsonObject(object: any): MsgQueueReceiveIn {
        return {
            topic: object['topic'],
            batchSize: object['batch_size'],
            leaseDurationMillis: object['lease_duration_millis'],
        };
    },

    _toJsonObject(self: MsgQueueReceiveIn): any {
        return {
            'topic': self.topic,
            'batch_size': self.batchSize,
            'lease_duration_millis': self.leaseDurationMillis,
        };
    }
}