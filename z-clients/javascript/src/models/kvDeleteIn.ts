// this file is @generated

export interface KvDeleteIn {
    namespace?: string | null;
}

export interface KvDeleteIn_ {
    namespace?: string | null;
    key: string;
}

export const KvDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvDeleteIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvDeleteIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
        };
    }
}