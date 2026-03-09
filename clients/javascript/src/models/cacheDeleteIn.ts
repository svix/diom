// this file is @generated

export interface CacheDeleteIn {
    key: string;
}

export const CacheDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheDeleteIn {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheDeleteIn): any {
        return {
            'key': self.key,
        };
    }
}