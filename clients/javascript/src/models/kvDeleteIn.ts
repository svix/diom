// this file is @generated

export interface KvDeleteIn {
    key: string;
}

export const KvDeleteInSerializer = {
    _fromJsonObject(object: any): KvDeleteIn {
        return {
            key: object['key'],
        };
    },

    _toJsonObject(self: KvDeleteIn): any {
        return {
            'key': self.key,
        };
    }
}