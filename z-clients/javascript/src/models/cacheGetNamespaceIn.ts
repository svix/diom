// this file is @generated

export interface CacheGetNamespaceIn {
    name: string;
}

export const CacheGetNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}