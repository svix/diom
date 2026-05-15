// this file is @generated

export interface MsgStreamCancelLeaseIn {
    namespace?: string | null;
}

export interface MsgStreamCancelLeaseIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
}

export const MsgStreamCancelLeaseInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamCancelLeaseIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamCancelLeaseIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
        };
    }
}