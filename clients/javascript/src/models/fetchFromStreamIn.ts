// this file is @generated





export interface FetchFromStreamIn {
    /** How many messages to read from the stream. */
    batchSize: number;
consumerGroup: string;
name: string;
/** How long messages are locked by the consumer. */
    visibilityTimeoutSeconds: number;
}

export const FetchFromStreamInSerializer = {
    _fromJsonObject(object: any): FetchFromStreamIn {
        return {
            batchSize: object['batch_size'],
            consumerGroup: object['consumer_group'],
            name: object['name'],
            visibilityTimeoutSeconds: object['visibility_timeout_seconds'],
            };
    },

    _toJsonObject(self: FetchFromStreamIn): any {
        return {
            'batch_size': self.batchSize,
            'consumer_group': self.consumerGroup,
            'name': self.name,
            'visibility_timeout_seconds': self.visibilityTimeoutSeconds,
            };
    }
}