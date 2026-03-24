// this file is @generated

export interface CacheGetOut {
    /** Time of expiry */
    expiry?: Date | null;
    value?: number[] | null;
}

export const CacheGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetOut {
        return {
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetOut): any {
        return {
            'expiry': self.expiry,
            'value': self.value,
        };
    }
}