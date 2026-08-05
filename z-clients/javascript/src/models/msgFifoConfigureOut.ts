// this file is @generated

export interface MsgFifoConfigureOut {
    retrySchedule: number[];
    dlqTopic?: string | null;
}

export const MsgFifoConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgFifoConfigureOut {
        return {
            retrySchedule: object['retry_schedule'],
            dlqTopic: object['dlq_topic'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgFifoConfigureOut): any {
        return {
            'retry_schedule': self.retrySchedule,
            'dlq_topic': self.dlqTopic,
        };
    }
}