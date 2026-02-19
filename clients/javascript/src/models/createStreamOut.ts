// this file is @generated





export interface CreateStreamOut {
    createdAt: Date;
maxByteSize?: number | null;
name: string;
retentionPeriodSeconds?: number | null;
updatedAt: Date;
}

export const CreateStreamOutSerializer = {
    _fromJsonObject(object: any): CreateStreamOut {
        return {
            createdAt: new Date(object['created_at']),
            maxByteSize: object['max_byte_size'],
            name: object['name'],
            retentionPeriodSeconds: object['retention_period_seconds'],
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: CreateStreamOut): any {
        return {
            'created_at': self.createdAt,
            'max_byte_size': self.maxByteSize,
            'name': self.name,
            'retention_period_seconds': self.retentionPeriodSeconds,
            'updated_at': self.updatedAt,
            };
    }
}