// this file is @generated

export interface CacheGetOut {
    /** Time of expiry */
    expiry?: Date | null;
    value?: Uint8Array | null;
}

export const CacheGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetOut {
        return {
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'] != null ? new Uint8Array(object['value']) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetOut): any {
        return {
            'expiry': self.expiry != null ? self.expiry.getTime() : undefined,
            'value': self.value != null ? Array.from(self.value) : undefined,
        };
    }
}