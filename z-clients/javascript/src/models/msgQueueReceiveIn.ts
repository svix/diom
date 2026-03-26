// this file is @generated

export interface MsgQueueReceiveIn {
    namespace?: string | null;
    batchSize?: number;
    leaseDurationMs?: number;
    /** Maximum time (in milliseconds) to wait for messages before returning. */
    batchWaitMs?: number | null;
}

export interface MsgQueueReceiveIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    batchSize?: number;
    leaseDurationMs?: number;
    /** Maximum time (in milliseconds) to wait for messages before returning. */
    batchWaitMs?: number | null;
}

export const MsgQueueReceiveInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueReceiveIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            batchSize: object['batch_size'],
            leaseDurationMs: object['lease_duration_ms'],
            batchWaitMs: object['batch_wait_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueReceiveIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'batch_size': self.batchSize,
            'lease_duration_ms': self.leaseDurationMs,
            'batch_wait_ms': self.batchWaitMs,
        };
    }
}