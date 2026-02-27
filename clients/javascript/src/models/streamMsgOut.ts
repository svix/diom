// this file is @generated





export interface StreamMsgOut {
    headers?: { [key: string]: string };
offset: number;
timestamp: Date;
topic: string;
value: number[];
}

export const StreamMsgOutSerializer = {
    _fromJsonObject(object: any): StreamMsgOut {
        return {
            headers: object['headers'],
            offset: object['offset'],
            timestamp: new Date(object['timestamp']),
            topic: object['topic'],
            value: object['value'],
            };
    },

    _toJsonObject(self: StreamMsgOut): any {
        return {
            'headers': self.headers,
            'offset': self.offset,
            'timestamp': self.timestamp,
            'topic': self.topic,
            'value': self.value,
            };
    }
}