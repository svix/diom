// this file is @generated

export interface MsgTopicConfigureIn {
    partitions: number;
}

export interface MsgTopicConfigureIn_ {
    topic: string;
    partitions: number;
}

export const MsgTopicConfigureInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicConfigureIn_ {
        return {
            topic: object['topic'],
            partitions: object['partitions'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicConfigureIn_): any {
        return {
            'topic': self.topic,
            'partitions': self.partitions,
        };
    }
}