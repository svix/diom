// this file is @generated

export interface CacheGetNamespaceIn {
    name: string;
}

export const CacheGetNamespaceInSerializer = {
    _fromJsonObject(object: any): CacheGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    _toJsonObject(self: CacheGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}