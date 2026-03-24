// this file is @generated

export interface MsgQueueConfigureOut {
    retrySchedule: number[];
    dlqTopic?: string | null;
}

export const MsgQueueConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueConfigureOut {
        return {
            retrySchedule: object['retry_schedule'],
            dlqTopic: object['dlq_topic'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueConfigureOut): any {
        return {
            'retry_schedule': self.retrySchedule,
            'dlq_topic': self.dlqTopic,
        };
    }
}