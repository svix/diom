// this file is @generated
// biome-ignore-all lint/suspicious/noEmptyInterface: forwards compat

export interface IdempotencyAbortIn {
}

export interface IdempotencyAbortIn_ {
    key: string;
}

export const IdempotencyAbortInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyAbortIn_ {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyAbortIn_): any {
        return {
            'key': self.key,
        };
    }
}