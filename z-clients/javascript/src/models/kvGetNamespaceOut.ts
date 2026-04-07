// this file is @generated

export interface KvGetNamespaceOut {
    name: string;
    created: number;
    updated: number;
}

export const KvGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetNamespaceOut {
        return {
            name: object['name'],
            created: object['created'],
            updated: object['updated'],
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