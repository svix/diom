// this file is @generated

export interface KvGetNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const KvGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetNamespaceOut {
        return {
            name: object['name'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created,
            'updated': self.updated,
        };
    }
}