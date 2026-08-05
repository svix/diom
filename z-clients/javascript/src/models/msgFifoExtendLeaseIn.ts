// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface MsgFifoExtendLeaseIn {
    namespace?: string | null;
    msgIds: string[];
    leaseDuration?: Temporal.Duration;
}

export interface MsgFifoExtendLeaseIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    msgIds: string[];
    leaseDuration?: Temporal.Duration;
}

export const MsgFifoExtendLeaseInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgFifoExtendLeaseIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            msgIds: object['msg_ids'],
            leaseDuration: object['lease_duration_ms'] != null ? Temporal.Duration.from({ milliseconds: object['lease_duration_ms'] }) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgFifoExtendLeaseIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'msg_ids': self.msgIds,
            'lease_duration_ms': self.leaseDuration != null ? self.leaseDuration.total('millisecond') : undefined,
        };
    }
}