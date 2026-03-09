// this file is @generated

export interface CacheDeleteOut {
    deleted: boolean;
}

export const CacheDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheDeleteOut {
        return {
            deleted: object['deleted'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheDeleteOut): any {
        return {
            'deleted': self.deleted,
        };
    }
}