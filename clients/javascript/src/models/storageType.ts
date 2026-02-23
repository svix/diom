// this file is @generated

export enum StorageType {
    Persistent = 'Persistent',
    Ephemeral = 'Ephemeral',
    }

export const StorageTypeSerializer = {
    _fromJsonObject(object: any): StorageType {
        return object;
    },

    _toJsonObject(self: StorageType): any {
        return self;
    }
}