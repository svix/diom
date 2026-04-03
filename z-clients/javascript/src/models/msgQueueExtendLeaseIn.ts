// this file is @generated

export interface MsgQueueExtendLeaseIn {
    namespace?: string | null;
    msgIds: string[];
    leaseDurationMs?: number;
}

export interface MsgQueueExtendLeaseIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    msgIds: string[];
    leaseDurationMs?: number;
}

export const MsgQueueExtendLeaseInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueExtendLeaseIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            msgIds: object['msg_ids'],
            leaseDurationMs: object['lease_duration_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueExtendLeaseIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'msg_ids': self.msgIds,
            'lease_duration_ms': self.leaseDurationMs,
        };
    }
}