// this file is @generated

export interface IdempotencyGetNamespaceIn {
    name: string;
}

export const IdempotencyGetNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}