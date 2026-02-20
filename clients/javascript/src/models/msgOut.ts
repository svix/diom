// this file is @generated





export interface MsgOut {
    headers: { [key: string]: string };
id: number;
payload: number[];
timestamp: Date;
}

export const MsgOutSerializer = {
    _fromJsonObject(object: any): MsgOut {
        return {
            headers: object['headers'],
            id: object['id'],
            payload: object['payload'],
            timestamp: new Date(object['timestamp']),
            };
    },

    _toJsonObject(self: MsgOut): any {
        return {
            'headers': self.headers,
            'id': self.id,
            'payload': self.payload,
            'timestamp': self.timestamp,
            };
    }
}