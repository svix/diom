// this file is @generated

export interface KvGetOut {
    key: string;
    /** Time of expiry */
    expiry?: Date | null;
    value: number[];
}

export const KvGetOutSerializer = {
    _fromJsonObject(object: any): KvGetOut {
        return {
            key: object['key'],
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'],
        };
    },

    _toJsonObject(self: KvGetOut): any {
        return {
            'key': self.key,
            'expiry': self.expiry,
            'value': self.value,
        };
    }
}