// this file is @generated

export interface IdempotencyAbortIn {
    key: string;
}

export const IdempotencyAbortInSerializer = {
    _fromJsonObject(object: any): IdempotencyAbortIn {
        return {
            key: object['key'],
            };
    },

    _toJsonObject(self: IdempotencyAbortIn): any {
        return {
            'key': self.key,
            };
    }
}