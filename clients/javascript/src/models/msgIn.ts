// this file is @generated





export interface MsgIn {
    headers?: { [key: string]: string };
payload: number[];
}

export const MsgInSerializer = {
    _fromJsonObject(object: any): MsgIn {
        return {
            headers: object['headers'],
            payload: object['payload'],
            };
    },

    _toJsonObject(self: MsgIn): any {
        return {
            'headers': self.headers,
            'payload': self.payload,
            };
    }
}