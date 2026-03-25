// this file is @generated

export interface KvCreateNamespaceIn {
    name: string;
    maxStorageBytes?: number | null;
}

export const KvCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvCreateNamespaceIn {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvCreateNamespaceIn): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
        };
    }
}