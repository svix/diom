// this file is @generated

export interface MsgTopicPartitionDescribeOut {
    partitionId: number;
    /** The next offset to be committed to this partition */
    highWaterMark: number;
}

export const MsgTopicPartitionDescribeOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicPartitionDescribeOut {
        return {
            partitionId: object['partition_id'],
            highWaterMark: object['high_water_mark'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicPartitionDescribeOut): any {
        return {
            'partition_id': self.partitionId,
            'high_water_mark': self.highWaterMark,
        };
    }
}