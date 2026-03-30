// this file is @generated

export interface KvCreateNamespaceIn {
    name: string;
}

export const KvCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvCreateNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvCreateNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}