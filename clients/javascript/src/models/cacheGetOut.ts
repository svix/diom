// this file is @generated





export interface CacheGetOut {
    /** Time of expiry */
    expiry?: Date | null;
key: string;
value: number[];
}

export const CacheGetOutSerializer = {
    _fromJsonObject(object: any): CacheGetOut {
        return {
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            key: object['key'],
            value: object['value'],
            };
    },

    _toJsonObject(self: CacheGetOut): any {
        return {
            'expiry': self.expiry,
            'key': self.key,
            'value': self.value,
            };
    }
}