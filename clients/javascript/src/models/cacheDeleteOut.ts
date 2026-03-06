// this file is @generated

export interface CacheDeleteOut {
    deleted: boolean;
}

export const CacheDeleteOutSerializer = {
    _fromJsonObject(object: any): CacheDeleteOut {
        return {
            deleted: object['deleted'],
            };
    },

    _toJsonObject(self: CacheDeleteOut): any {
        return {
            'deleted': self.deleted,
            };
    }
}