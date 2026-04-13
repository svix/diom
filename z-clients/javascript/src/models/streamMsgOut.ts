// this file is @generated

export interface StreamMsgOut {
    offset: number;
    topic: string;
    value: Uint8Array;
    headers: { [key: string]: string };
    timestamp: Date;
    scheduledAt?: Date | null;
}

export const StreamMsgOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): StreamMsgOut {
        return {
            offset: object['offset'],
            topic: object['topic'],
            value: new Uint8Array(object['value']),
            headers: object['headers'],
            timestamp: new Date(object['timestamp']),
            scheduledAt: object['scheduled_at'] ? new Date(object['scheduled_at']) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: StreamMsgOut): any {
        return {
            'offset': self.offset,
            'topic': self.topic,
            'value': Array.from(self.value),
            'headers': self.headers,
            'timestamp': self.timestamp,
            'scheduled_at': self.scheduledAt,
        };
    }
}