// this file is @generated

export interface IdempotencyStartIn {
    namespace?: string | null;
    /** How long to hold the lock on start before releasing it. */
    lockPeriod: Date;
}

export interface IdempotencyStartIn_ {
    namespace?: string | null;
    key: string;
    /** How long to hold the lock on start before releasing it. */
    lockPeriod: Date;
}

export const IdempotencyStartInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyStartIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            lockPeriod: new Date(object['lock_period_ms']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyStartIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'lock_period_ms': self.lockPeriod,
        };
    }
}