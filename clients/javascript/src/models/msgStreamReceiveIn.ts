// this file is @generated

export interface MsgStreamReceiveIn {
    namespace?: string | null;
    batchSize?: number;
    leaseDurationMillis?: number;
    defaultStartingPosition?: string;
}

export interface MsgStreamReceiveIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    batchSize?: number;
    leaseDurationMillis?: number;
    defaultStartingPosition?: string;
}

export const MsgStreamReceiveInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamReceiveIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            batchSize: object['batch_size'],
            leaseDurationMillis: object['lease_duration_millis'],
            defaultStartingPosition: object['default_starting_position'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamReceiveIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'batch_size': self.batchSize,
            'lease_duration_millis': self.leaseDurationMillis,
            'default_starting_position': self.defaultStartingPosition,
        };
    }
}