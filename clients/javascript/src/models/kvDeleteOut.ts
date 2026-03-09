// this file is @generated

export interface KvDeleteOut {
    deleted: boolean;
}

export const KvDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvDeleteOut {
        return {
            deleted: object['deleted'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvDeleteOut): any {
        return {
            'deleted': self.deleted,
        };
    }
}