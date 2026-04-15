// this file is @generated

export interface IdempotencyConfigureNamespaceIn {
    name: string;
}

export const IdempotencyConfigureNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyConfigureNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyConfigureNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}