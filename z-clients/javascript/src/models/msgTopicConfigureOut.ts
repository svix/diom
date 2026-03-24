// this file is @generated

export interface MsgTopicConfigureOut {
    partitions: number;
}

export const MsgTopicConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicConfigureOut {
        return {
            partitions: object['partitions'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicConfigureOut): any {
        return {
            'partitions': self.partitions,
        };
    }
}