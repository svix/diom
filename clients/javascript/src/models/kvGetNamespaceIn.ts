// this file is @generated

export interface KvGetNamespaceIn {
    name: string;
}

export const KvGetNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}