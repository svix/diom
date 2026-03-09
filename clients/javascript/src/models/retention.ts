// this file is @generated

export interface Retention {
    millis?: number;
    bytes?: number;
}

export const RetentionSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): Retention {
        return {
            millis: object['millis'],
            bytes: object['bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: Retention): any {
        return {
            'millis': self.millis,
            'bytes': self.bytes,
        };
    }
}