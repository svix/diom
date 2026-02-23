// this file is @generated





export interface IdempotencyGetGroupIn {
    name: string;
}

export const IdempotencyGetGroupInSerializer = {
    _fromJsonObject(object: any): IdempotencyGetGroupIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: IdempotencyGetGroupIn): any {
        return {
            'name': self.name,
            };
    }
}