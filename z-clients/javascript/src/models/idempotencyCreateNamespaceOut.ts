// this file is @generated

export interface IdempotencyCreateNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const IdempotencyCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCreateNamespaceOut {
        return {
            name: object['name'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCreateNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}