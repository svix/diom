// this file is @generated

export interface KvGetIn {
    key: string;
}

export const KvGetInSerializer = {
    _fromJsonObject(object: any): KvGetIn {
        return {
            key: object['key'],
        };
    },

    _toJsonObject(self: KvGetIn): any {
        return {
            'key': self.key,
        };
    }
}