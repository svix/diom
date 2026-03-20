// this file is @generated

export interface CacheDeleteIn {
    namespace?: string | null;
}

export interface CacheDeleteIn_ {
    namespace?: string | null;
    key: string;
}

export const CacheDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheDeleteIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheDeleteIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
        };
    }
}