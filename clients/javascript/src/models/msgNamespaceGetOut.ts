// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface MsgNamespaceGetOut {
    name: string;
    retention: Retention;
    storageType: StorageType;
    created: Date;
    updated: Date;
}

export const MsgNamespaceGetOutSerializer = {
    _fromJsonObject(object: any): MsgNamespaceGetOut {
        return {
            name: object['name'],
            retention: RetentionSerializer._fromJsonObject(object['retention']),
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
            };
    },

    _toJsonObject(self: MsgNamespaceGetOut): any {
        return {
            'name': self.name,
            'retention': RetentionSerializer._toJsonObject(self.retention),
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'created': self.created,
            'updated': self.updated,
            };
    }
}