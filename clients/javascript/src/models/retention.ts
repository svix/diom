// this file is @generated





export interface Retention {
    bytes?: number;
millis?: number;
}

export const RetentionSerializer = {
    _fromJsonObject(object: any): Retention {
        return {
            bytes: object['bytes'],
            millis: object['millis'],
            };
    },

    _toJsonObject(self: Retention): any {
        return {
            'bytes': self.bytes,
            'millis': self.millis,
            };
    }
}