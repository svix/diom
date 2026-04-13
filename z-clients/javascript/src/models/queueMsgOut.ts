// this file is @generated

export interface QueueMsgOut {
    msgId: string;
    value: Uint8Array;
    headers: { [key: string]: string };
    timestamp: Date;
    scheduledAt?: Date | null;
}

export const QueueMsgOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): QueueMsgOut {
        return {
            msgId: object['msg_id'],
            value: new Uint8Array(object['value']),
            headers: object['headers'],
            timestamp: new Date(Number(object['timestamp'])),
            scheduledAt: object['scheduled_at'] ? new Date(Number(object['scheduled_at'])) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: QueueMsgOut): any {
        return {
            'msg_id': self.msgId,
            'value': Array.from(self.value),
            'headers': self.headers,
            'timestamp': self.timestamp.getTime(),
            'scheduled_at': self.scheduledAt != null ? self.scheduledAt.getTime() : undefined,
        };
    }
}