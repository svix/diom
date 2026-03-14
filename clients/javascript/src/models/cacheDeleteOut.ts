// this file is @generated

export interface CacheDeleteOut {
    success: boolean;
}

export const CacheDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheDeleteOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheDeleteOut): any {
        return {
            'success': self.success,
        };
    }
}