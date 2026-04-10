// this file is @generated

export interface Retention {
    period?: Date | null;
}

export const RetentionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Retention {
        return {
            period: object['period_ms'] ? new Date(object['period_ms']) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Retention): any {
        return {
            'period_ms': self.period != null ? self.period.getTime() : undefined,
        };
    }
}