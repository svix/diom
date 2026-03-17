// this file is @generated
// biome-ignore-all lint/suspicious/noEmptyInterface: forwards compat

export interface CacheDeleteIn {
}

export interface CacheDeleteIn_ {
    key: string;
}

export const CacheDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheDeleteIn_ {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheDeleteIn_): any {
        return {
            'key': self.key,
        };
    }
}