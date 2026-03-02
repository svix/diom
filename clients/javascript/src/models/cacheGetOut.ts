// this file is @generated





export interface CacheGetOut {
    key: string;
/** Time of expiry */
    expiry?: Date | null;
value: number[];
}

export const CacheGetOutSerializer = {
    _fromJsonObject(object: any): CacheGetOut {
        return {
            key: object['key'],
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'],
            };
    },

    _toJsonObject(self: CacheGetOut): any {
        return {
            'key': self.key,
            'expiry': self.expiry,
            'value': self.value,
            };
    }
}