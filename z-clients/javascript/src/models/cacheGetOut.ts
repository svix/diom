// this file is @generated

export interface CacheGetOut {
    /** Time of expiry */
    expiry?: number | null;
    value?: Uint8Array | null;
}

export const CacheGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetOut {
        return {
            expiry: object['expiry'],
            value: object['value'] != null ? new Uint8Array(object['value']) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetOut): any {
        return {
            'expiry': self.expiry,
            'value': self.value != null ? Array.from(self.value) : undefined,
        };
    }
}