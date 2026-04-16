// this file is @generated

export interface KvSetOut {
    version: number;
}

export const KvSetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvSetOut {
        return {
            version: object['version'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvSetOut): any {
        return {
            'version': self.version,
        };
    }
}