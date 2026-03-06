// this file is @generated

export interface CacheDeleteIn {
    key: string;
}

export const CacheDeleteInSerializer = {
    _fromJsonObject(object: any): CacheDeleteIn {
        return {
            key: object['key'],
            };
    },

    _toJsonObject(self: CacheDeleteIn): any {
        return {
            'key': self.key,
            };
    }
}