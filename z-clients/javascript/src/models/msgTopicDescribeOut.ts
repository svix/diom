// this file is @generated
import {
    type MsgTopicPartitionDescribeOut,
    MsgTopicPartitionDescribeOutSerializer,
} from './msgTopicPartitionDescribeOut';

export interface MsgTopicDescribeOut {
    /**
     * The unique internal ID of this topic
     * 
     * This can useful for debugging
     */
    id: string;
    name: string;
    partitions: MsgTopicPartitionDescribeOut[];
}

export const MsgTopicDescribeOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicDescribeOut {
        return {
            id: object['id'],
            name: object['name'],
            partitions: object['partitions'].map((item: MsgTopicPartitionDescribeOut) => MsgTopicPartitionDescribeOutSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicDescribeOut): any {
        return {
            'id': self.id,
            'name': self.name,
            'partitions': self.partitions.map((item) => MsgTopicPartitionDescribeOutSerializer._toJsonObject(item)),
        };
    }
}