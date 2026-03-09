// this file is @generated

export interface MsgQueueReceiveIn {
    topic: string;
    batchSize?: number;
    leaseDurationMillis?: number;
}

export const MsgQueueReceiveInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueReceiveIn {
        return {
            topic: object['topic'],
            batchSize: object['batch_size'],
            leaseDurationMillis: object['lease_duration_millis'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueReceiveIn): any {
        return {
            'topic': self.topic,
            'batch_size': self.batchSize,
            'lease_duration_millis': self.leaseDurationMillis,
        };
    }
}