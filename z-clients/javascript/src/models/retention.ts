// this file is @generated

export interface Retention {
    ms?: number;
    bytes?: number;
}

export const RetentionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Retention {
        return {
            ms: object['ms'],
            bytes: object['bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Retention): any {
        return {
            'ms': self.ms,
            'bytes': self.bytes,
        };
    }
}