// this file is @generated





export interface IdempotencyGetNamespaceIn {
    name: string;
}

export const IdempotencyGetNamespaceInSerializer = {
    _fromJsonObject(object: any): IdempotencyGetNamespaceIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: IdempotencyGetNamespaceIn): any {
        return {
            'name': self.name,
            };
    }
}