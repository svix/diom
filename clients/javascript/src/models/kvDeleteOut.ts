// this file is @generated

export interface KvDeleteOut {
    /** Whether the operation succeeded or was a noop due to pre-conditions. */
    success: boolean;
}

export const KvDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvDeleteOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvDeleteOut): any {
        return {
            'success': self.success,
        };
    }
}