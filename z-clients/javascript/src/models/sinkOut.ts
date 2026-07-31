// this file is @generated
import {
    type SeekPosition,
    SeekPositionSerializer,
} from './seekPosition';
import {
    type SinkConfig,
    SinkConfigSerializer,
} from './sinkConfig';

export interface SinkOut {
    topic: string;
    consumerGroup: string;
    /** Where a freshly-created sink starts consuming the topic. Defaults to `earliest`. */
    defaultStartingPosition?: SeekPosition;
    /** At most how many concurrent requests will be sent to the Sink. */
    maxInFlight?: number | null;
    config: SinkConfig;
}

export const SinkOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): SinkOut {
        return {
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            defaultStartingPosition: object['default_starting_position'] != null ? SeekPositionSerializer._fromJsonObject(object['default_starting_position']): undefined,
            maxInFlight: object['max_in_flight'],
            config: SinkConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: SinkOut): any {
        return {
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'default_starting_position': self.defaultStartingPosition != null ? SeekPositionSerializer._toJsonObject(self.defaultStartingPosition) : undefined,
            'max_in_flight': self.maxInFlight,
            'config': SinkConfigSerializer._toJsonObject(self.config),
        };
    }
}