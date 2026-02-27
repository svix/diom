// this file is @generated





export interface MsgIn {
    headers?: { [key: string]: string };
/** Optional partition key. Messages with the same key are routed to the same partition. */
    key?: string | null;
value: number[];
}

export const MsgInSerializer = {
    _fromJsonObject(object: any): MsgIn {
        return {
            headers: object['headers'],
            key: object['key'],
            value: object['value'],
            };
    },

    _toJsonObject(self: MsgIn): any {
        return {
            'headers': self.headers,
            'key': self.key,
            'value': self.value,
            };
    }
}