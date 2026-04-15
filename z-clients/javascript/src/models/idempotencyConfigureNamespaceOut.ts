// this file is @generated

export interface IdempotencyConfigureNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const IdempotencyConfigureNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyConfigureNamespaceOut {
        return {
            name: object['name'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyConfigureNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}