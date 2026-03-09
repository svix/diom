// this file is @generated

export interface KvGetIn {
    key: string;
}

export const KvGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetIn {
        return {
            key: object['key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetIn): any {
        return {
            'key': self.key,
        };
    }
}