// this file is @generated

export interface KvDeleteIn {
    key: string;
}

export const KvDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvDeleteIn {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvDeleteIn): any {
        return {
            'key': self.key,
        };
    }
}