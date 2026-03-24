// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';

export interface MsgNamespaceCreateIn {
    retention?: Retention;
    storageType?: StorageType;
}

export interface MsgNamespaceCreateIn_ {
    name: string;
    retention?: Retention;
    storageType?: StorageType;
}

export const MsgNamespaceCreateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceCreateIn_ {
        return {
            name: object['name'],
            retention: object['retention'] != null ? RetentionSerializer._fromJsonObject(object['retention']): undefined,
            storageType: object['storage_type'] != null ? StorageTypeSerializer._fromJsonObject(object['storage_type']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceCreateIn_): any {
        return {
            'name': self.name,
            'retention': self.retention != null ? RetentionSerializer._toJsonObject(self.retention) : undefined,
            'storage_type': self.storageType != null ? StorageTypeSerializer._toJsonObject(self.storageType) : undefined,
        };
    }
}