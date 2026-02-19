// this file is @generated





export interface CacheSetIn {
    key: string;
/** Time to live in milliseconds */
    ttl: number;
value: number[];
}

export const CacheSetInSerializer = {
    _fromJsonObject(object: any): CacheSetIn {
        return {
            key: object['key'],
            ttl: object['ttl'],
            value: object['value'],
            };
    },

    _toJsonObject(self: CacheSetIn): any {
        return {
            'key': self.key,
            'ttl': self.ttl,
            'value': self.value,
            };
    }
}