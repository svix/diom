// this file is @generated

export interface KvCreateNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const KvCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvCreateNamespaceOut {
        return {
            name: object['name'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvCreateNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}