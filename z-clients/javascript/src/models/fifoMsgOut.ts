// this file is @generated

export interface FifoMsgOut {
    msgId: string;
    key?: string | null;
    value: Uint8Array;
    headers: { [key: string]: string };
    timestamp: Date;
    scheduledAt?: Date | null;
}

export const FifoMsgOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): FifoMsgOut {
        return {
            msgId: object['msg_id'],
            key: object['key'],
            value: new Uint8Array(object['value']),
            headers: object['headers'],
            timestamp: new Date(Number(object['timestamp'])),
            scheduledAt: object['scheduled_at'] ? new Date(Number(object['scheduled_at'])) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: FifoMsgOut): any {
        return {
            'msg_id': self.msgId,
            'key': self.key,
            'value': Array.from(self.value),
            'headers': self.headers,
            'timestamp': self.timestamp.getTime(),
            'scheduled_at': self.scheduledAt != null ? self.scheduledAt.getTime() : undefined,
        };
    }
}