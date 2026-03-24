// this file is @generated

export interface IdempotencyAbortIn {
    namespace?: string | null;
}

export interface IdempotencyAbortIn_ {
    namespace?: string | null;
    key: string;
}

export const IdempotencyAbortInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyAbortIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyAbortIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
        };
    }
}