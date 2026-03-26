// this file is @generated
import {
    type SeekPosition,
    SeekPositionSerializer,
} from './seekPosition';

export interface MsgStreamReceiveIn {
    namespace?: string | null;
    batchSize?: number;
    leaseDurationMs?: number;
    defaultStartingPosition?: SeekPosition;
    /** Maximum time (in milliseconds) to wait for messages before returning. */
    batchWaitMs?: number | null;
}

export interface MsgStreamReceiveIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    batchSize?: number;
    leaseDurationMs?: number;
    defaultStartingPosition?: SeekPosition;
    /** Maximum time (in milliseconds) to wait for messages before returning. */
    batchWaitMs?: number | null;
}

export const MsgStreamReceiveInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamReceiveIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            batchSize: object['batch_size'],
            leaseDurationMs: object['lease_duration_ms'],
            defaultStartingPosition: object['default_starting_position'] != null ? SeekPositionSerializer._fromJsonObject(object['default_starting_position']): undefined,
            batchWaitMs: object['batch_wait_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamReceiveIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'batch_size': self.batchSize,
            'lease_duration_ms': self.leaseDurationMs,
            'default_starting_position': self.defaultStartingPosition != null ? SeekPositionSerializer._toJsonObject(self.defaultStartingPosition) : undefined,
            'batch_wait_ms': self.batchWaitMs,
        };
    }
}