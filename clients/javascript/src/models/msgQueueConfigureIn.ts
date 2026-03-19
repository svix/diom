// this file is @generated

export interface MsgQueueConfigureIn {
    namespace?: string | null;
    retrySchedule?: number[];
    dlqTopic?: string | null;
}

export interface MsgQueueConfigureIn_ {
    namespace?: string | null;
    topic: string;
    consumerGroup: string;
    retrySchedule?: number[];
    dlqTopic?: string | null;
}

export const MsgQueueConfigureInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueConfigureIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consumerGroup: object['consumer_group'],
            retrySchedule: object['retry_schedule'],
            dlqTopic: object['dlq_topic'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueConfigureIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consumer_group': self.consumerGroup,
            'retry_schedule': self.retrySchedule,
            'dlq_topic': self.dlqTopic,
        };
    }
}