// this file is @generated

export interface KvGetOut {
    /** Time of expiry */
    expiry?: Date | null;
    value?: number[] | null;
}

export const KvGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetOut {
        return {
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetOut): any {
        return {
            'expiry': self.expiry,
            'value': self.value,
        };
    }
}