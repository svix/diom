// this file is @generated





export interface KvGetOut {
    /** Time of expiry */
    expiry?: Date | null;
key: string;
value: number[];
}

export const KvGetOutSerializer = {
    _fromJsonObject(object: any): KvGetOut {
        return {
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            key: object['key'],
            value: object['value'],
            };
    },

    _toJsonObject(self: KvGetOut): any {
        return {
            'expiry': self.expiry,
            'key': self.key,
            'value': self.value,
            };
    }
}