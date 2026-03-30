// this file is @generated

export interface IdempotencyCreateNamespaceIn {
    name: string;
}

export const IdempotencyCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCreateNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCreateNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}