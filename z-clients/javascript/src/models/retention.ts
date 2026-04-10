// this file is @generated

export interface Retention {
    period?: Date | null;
}

export const RetentionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Retention {
        return {
            periodMs: object['period_ms'] ? new Date(object['period_ms']) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Retention): any {
        return {
            'period_ms': self.periodMs,
        };
    }
}