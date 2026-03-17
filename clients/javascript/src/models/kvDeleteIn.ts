// this file is @generated
// biome-ignore-all lint/suspicious/noEmptyInterface: forwards compat

export interface KvDeleteIn {
}

export interface KvDeleteIn_ {
    key: string;
}

export const KvDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvDeleteIn_ {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvDeleteIn_): any {
        return {
            'key': self.key,
        };
    }
}