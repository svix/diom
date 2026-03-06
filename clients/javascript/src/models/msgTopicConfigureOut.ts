// this file is @generated

export interface MsgTopicConfigureOut {
    partitions: number;
}

export const MsgTopicConfigureOutSerializer = {
    _fromJsonObject(object: any): MsgTopicConfigureOut {
        return {
            partitions: object['partitions'],
        };
    },

    _toJsonObject(self: MsgTopicConfigureOut): any {
        return {
            'partitions': self.partitions,
        };
    }
}