// this file is @generated





export interface CacheSetIn {
    key: string;
    value: number[];
    /** Time to live in milliseconds */
    ttl: number;
}

export const CacheSetInSerializer = {
    _fromJsonObject(object: any): CacheSetIn {
        return {
            key: object['key'],
            value: object['value'],
            ttl: object['ttl'],
            };
    },

    _toJsonObject(self: CacheSetIn): any {
        return {
            'key': self.key,
            'value': self.value,
            'ttl': self.ttl,
            };
    }
}