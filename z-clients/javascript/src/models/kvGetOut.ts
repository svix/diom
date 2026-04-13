// this file is @generated

export interface KvGetOut {
    /** Time of expiry */
    expiry?: Date | null;
    value?: Uint8Array | null;
    /**
     * Opaque version token for optimistic concurrency control.
     * Pass as `version` in a subsequent `set` to perform a conditional write.
     */
    version: number;
}

export const KvGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetOut {
        return {
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            value: object['value'] != null ? new Uint8Array(object['value']) : null,
            version: object['version'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetOut): any {
        return {
            'expiry': self.expiry != null ? self.expiry.getTime() : undefined,
            'value': self.value != null ? Array.from(self.value) : undefined,
            'version': self.version,
        };
    }
}