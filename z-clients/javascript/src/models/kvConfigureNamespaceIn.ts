// this file is @generated

export interface KvConfigureNamespaceIn {
    name: string;
}

export const KvConfigureNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvConfigureNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvConfigureNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}