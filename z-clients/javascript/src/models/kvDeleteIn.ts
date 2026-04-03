// this file is @generated

export interface KvDeleteIn {
    namespace?: string | null;
    /**
     * If set, the delete only succeeds when the stored version matches this value.
     * Use the `version` field from a prior `get` response.
     */
    version?: number | null;
}

export interface KvDeleteIn_ {
    namespace?: string | null;
    key: string;
    /**
     * If set, the delete only succeeds when the stored version matches this value.
     * Use the `version` field from a prior `get` response.
     */
    version?: number | null;
}

export const KvDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvDeleteIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            version: object['version'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvDeleteIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'version': self.version,
        };
    }
}