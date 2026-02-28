// this file is @generated





export interface MsgTopicConfigureIn {
    partitions: number;
topic: string;
}

export const MsgTopicConfigureInSerializer = {
    _fromJsonObject(object: any): MsgTopicConfigureIn {
        return {
            partitions: object['partitions'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: MsgTopicConfigureIn): any {
        return {
            'partitions': self.partitions,
            'topic': self.topic,
            };
    }
}