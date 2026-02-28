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
    created: Date;
name: string;
retention: Retention;
storageType: StorageType;
updated: Date;
}

export const MsgNamespaceGetOutSerializer = {
    _fromJsonObject(object: any): MsgNamespaceGetOut {
        return {
            created: new Date(object['created']),
            name: object['name'],
            retention: RetentionSerializer._fromJsonObject(object['retention']),
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            updated: new Date(object['updated']),
            };
    },

    _toJsonObject(self: MsgNamespaceGetOut): any {
        return {
            'created': self.created,
            'name': self.name,
            'retention': RetentionSerializer._toJsonObject(self.retention),
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'updated': self.updated,
            };
    }
}