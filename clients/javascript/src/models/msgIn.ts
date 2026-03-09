// this file is @generated

export interface MsgIn {
    value: number[];
    headers?: { [key: string]: string };
    /** Optional partition key. Messages with the same key are routed to the same partition. */
    key?: string | null;
}

export const MsgInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgIn {
        return {
            value: object['value'],
            headers: object['headers'],
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgIn): any {
        return {
            'value': self.value,
            'headers': self.headers,
            'key': self.key,
        };
    }
}