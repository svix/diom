// this file is @generated

export interface MsgQueueReceiveIn {
    namespace?: string | null;
    batchSize?: number;
    leaseDuration?: Date;
    /** Maximum time (in milliseconds) to wait for messages before returning. */
    batchWait?: Date | null;
}

export interface MsgQueueReceiveIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    batchSize?: number;
    leaseDuration?: Date;
    /** Maximum time (in milliseconds) to wait for messages before returning. */
    batchWait?: Date | null;
}

export const MsgQueueReceiveInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueReceiveIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            batchSize: object['batch_size'],
            leaseDuration: object['lease_duration_ms'] ? new Date(object['lease_duration_ms']) : undefined,
            batchWait: object['batch_wait_ms'] ? new Date(object['batch_wait_ms']) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueReceiveIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'batch_size': self.batchSize,
            'lease_duration_ms': self.leaseDuration != null ? self.leaseDuration.getTime() : undefined,
            'batch_wait_ms': self.batchWait != null ? self.batchWait.getTime() : undefined,
        };
    }
}