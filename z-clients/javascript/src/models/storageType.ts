// this file is @generated

export enum StorageType {
    Persistent = 'Persistent',
    Ephemeral = 'Ephemeral',
    }

export const StorageTypeSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): StorageType {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: StorageType): any {
        return self;
    }
}