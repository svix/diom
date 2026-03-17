// this file is @generated

export interface IdempotencyStartIn {
    /** TTL in seconds for the lock/response */
    ttl: number;
}

export interface IdempotencyStartIn_ {
    key: string;
    /** TTL in seconds for the lock/response */
    ttl: number;
}

export const IdempotencyStartInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyStartIn_ {
        return {
            key: object['key'],
            ttl: object['ttl'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyStartIn_): any {
        return {
            'key': self.key,
            'ttl': self.ttl,
        };
    }
}