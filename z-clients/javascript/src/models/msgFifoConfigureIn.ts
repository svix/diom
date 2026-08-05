// this file is @generated

export interface MsgFifoConfigureIn {
    namespace?: string | null;
    retrySchedule?: number[];
    dlqTopic?: string | null;
}

export interface MsgFifoConfigureIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    retrySchedule?: number[];
    dlqTopic?: string | null;
}

export const MsgFifoConfigureInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgFifoConfigureIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            retrySchedule: object['retry_schedule'],
            dlqTopic: object['dlq_topic'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgFifoConfigureIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'retry_schedule': self.retrySchedule,
            'dlq_topic': self.dlqTopic,
        };
    }
}