// this file is @generated

export interface Retention {
    periodMs?: number | null;
    sizeBytes?: number | null;
}

export const RetentionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Retention {
        return {
            periodMs: object['period_ms'],
            sizeBytes: object['size_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Retention): any {
        return {
            'period_ms': self.periodMs,
            'size_bytes': self.sizeBytes,
        };
    }
}