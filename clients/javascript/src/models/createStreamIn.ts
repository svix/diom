// this file is @generated





export interface CreateStreamIn {
    /** How many bytes in total the stream will retain before dropping data. */
    maxByteSize?: number | null;
name: string;
/** How long messages are retained in the stream before being permanently nuked. */
    retentionPeriodSeconds?: number | null;
}

export const CreateStreamInSerializer = {
    _fromJsonObject(object: any): CreateStreamIn {
        return {
            maxByteSize: object['max_byte_size'],
            name: object['name'],
            retentionPeriodSeconds: object['retention_period_seconds'],
            };
    },

    _toJsonObject(self: CreateStreamIn): any {
        return {
            'max_byte_size': self.maxByteSize,
            'name': self.name,
            'retention_period_seconds': self.retentionPeriodSeconds,
            };
    }
}