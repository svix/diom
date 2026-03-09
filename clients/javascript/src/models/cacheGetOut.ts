// this file is @generated

export interface CacheGetOut {
    key: string;
    /** Time of expiry */
    expiry?: Date | null;
    value: number[];
}

export const CacheGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetOut {
        return {
            key: object['key'],
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetOut): any {
        return {
            'key': self.key,
            'expiry': self.expiry,
            'value': self.value,
        };
    }
}