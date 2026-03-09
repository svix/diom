// this file is @generated

export interface IdempotencyAbortIn {
    key: string;
}

export const IdempotencyAbortInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyAbortIn {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyAbortIn): any {
        return {
            'key': self.key,
        };
    }
}