// this file is @generated

export interface CacheGetIn {
    key: string;
}

export const CacheGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetIn {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetIn): any {
        return {
            'key': self.key,
        };
    }
}