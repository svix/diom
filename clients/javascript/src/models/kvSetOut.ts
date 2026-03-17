// this file is @generated

export interface KvSetOut {
    /** Whether the operation succeeded or was a noop due to pre-conditions. */
    success: boolean;
}

export const KvSetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvSetOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvSetOut): any {
        return {
            'success': self.success,
        };
    }
}