// this file is @generated

export interface StreamMsgOut {
    offset: number;
    topic: string;
    value: number[];
    headers?: { [key: string]: string };
    timestamp: Date;
}

export const StreamMsgOutSerializer = {
    _fromJsonObject(object: any): StreamMsgOut {
        return {
            offset: object['offset'],
            topic: object['topic'],
            value: object['value'],
            headers: object['headers'],
            timestamp: new Date(object['timestamp']),
        };
    },

    _toJsonObject(self: StreamMsgOut): any {
        return {
            'offset': self.offset,
            'topic': self.topic,
            'value': self.value,
            'headers': self.headers,
            'timestamp': self.timestamp,
        };
    }
}