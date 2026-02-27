// this file is @generated





export interface MsgIn2 {
    headers?: { [key: string]: string };
/** Optional partition key. Messages with the same key are routed to the same partition. */
    key?: string | null;
value: number[];
}

export const MsgIn2Serializer = {
    _fromJsonObject(object: any): MsgIn2 {
        return {
            headers: object['headers'],
            key: object['key'],
            value: object['value'],
            };
    },

    _toJsonObject(self: MsgIn2): any {
        return {
            'headers': self.headers,
            'key': self.key,
            'value': self.value,
            };
    }
}