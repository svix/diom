// this file is @generated
import {
    type SeekPosition,
    SeekPositionSerializer,
} from './seekPosition';

export interface MsgStreamSeekIn {
    namespace?: string | null;
    offset?: number | null;
    position?: SeekPosition | null;
    timestamp?: Date | null;
}

export interface MsgStreamSeekIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    offset?: number | null;
    position?: SeekPosition | null;
    timestamp?: Date | null;
}

export const MsgStreamSeekInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamSeekIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            offset: object['offset'],
            position: object['position'] != null ? SeekPositionSerializer._fromJsonObject(object['position']): undefined,
            timestamp: object['timestamp'] ? new Date(Number(object['timestamp'])) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamSeekIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'offset': self.offset,
            'position': self.position != null ? SeekPositionSerializer._toJsonObject(self.position) : undefined,
            'timestamp': self.timestamp != null ? self.timestamp.getTime() : undefined,
        };
    }
}