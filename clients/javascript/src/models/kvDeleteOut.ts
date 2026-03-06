// this file is @generated

export interface KvDeleteOut {
    deleted: boolean;
}

export const KvDeleteOutSerializer = {
    _fromJsonObject(object: any): KvDeleteOut {
        return {
            deleted: object['deleted'],
        };
    },

    _toJsonObject(self: KvDeleteOut): any {
        return {
            'deleted': self.deleted,
        };
    }
}