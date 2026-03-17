// this file is @generated

export interface CacheSetIn {
    value: number[];
    /** Time to live in milliseconds */
    ttl: number;
}

export interface CacheSetIn_ {
    key: string;
    value: number[];
    /** Time to live in milliseconds */
    ttl: number;
}

export const CacheSetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheSetIn_ {
        return {
            key: object['key'],
            value: object['value'],
            ttl: object['ttl'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheSetIn_): any {
        return {
            'key': self.key,
            'value': self.value,
            'ttl': self.ttl,
        };
    }
}