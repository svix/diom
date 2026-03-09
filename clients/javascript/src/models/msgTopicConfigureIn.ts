// this file is @generated

export interface MsgTopicConfigureIn {
    topic: string;
    partitions: number;
}

export const MsgTopicConfigureInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicConfigureIn {
        return {
            topic: object['topic'],
            partitions: object['partitions'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicConfigureIn): any {
        return {
            'topic': self.topic,
            'partitions': self.partitions,
        };
    }
}