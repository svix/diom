// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface GetStreamOut {
    createdAt: Date;
maxByteSize?: number | null;
name: string;
retentionPeriodSeconds?: number | null;
storageType: StorageType;
updatedAt: Date;
}

export const GetStreamOutSerializer = {
    _fromJsonObject(object: any): GetStreamOut {
        return {
            createdAt: new Date(object['created_at']),
            maxByteSize: object['max_byte_size'],
            name: object['name'],
            retentionPeriodSeconds: object['retention_period_seconds'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: GetStreamOut): any {
        return {
            'created_at': self.createdAt,
            'max_byte_size': self.maxByteSize,
            'name': self.name,
            'retention_period_seconds': self.retentionPeriodSeconds,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'updated_at': self.updatedAt,
            };
    }
}