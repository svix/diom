// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface Retention {
    period?: Temporal.Duration | null;
}

export const RetentionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Retention {
        return {
            period: object['period_ms'] != null ? Temporal.Duration.from({ milliseconds: object['period_ms'] }) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Retention): any {
        return {
            'period_ms': self.period != null ? self.period.total('millisecond') : undefined,
        };
    }
}