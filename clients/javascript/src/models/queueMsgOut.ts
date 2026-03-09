// this file is @generated

export interface QueueMsgOut {
    msgId: string;
    value: number[];
    headers?: { [key: string]: string };
    timestamp: Date;
}

export const QueueMsgOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): QueueMsgOut {
        return {
            msgId: object['msg_id'],
            value: object['value'],
            headers: object['headers'],
            timestamp: new Date(object['timestamp']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: QueueMsgOut): any {
        return {
            'msg_id': self.msgId,
            'value': self.value,
            'headers': self.headers,
            'timestamp': self.timestamp,
        };
    }
}