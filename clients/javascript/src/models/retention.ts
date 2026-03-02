// this file is @generated





export interface Retention {
    millis?: number;
bytes?: number;
}

export const RetentionSerializer = {
    _fromJsonObject(object: any): Retention {
        return {
            millis: object['millis'],
            bytes: object['bytes'],
            };
    },

    _toJsonObject(self: Retention): any {
        return {
            'millis': self.millis,
            'bytes': self.bytes,
            };
    }
}