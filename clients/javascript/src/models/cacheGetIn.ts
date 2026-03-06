// this file is @generated

export interface CacheGetIn {
    key: string;
}

export const CacheGetInSerializer = {
    _fromJsonObject(object: any): CacheGetIn {
        return {
            key: object['key'],
        };
    },

    _toJsonObject(self: CacheGetIn): any {
        return {
            'key': self.key,
        };
    }
}