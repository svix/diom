// this file is @generated





export interface MsgTopicConfigureIn {
    topic: string;
partitions: number;
}

export const MsgTopicConfigureInSerializer = {
    _fromJsonObject(object: any): MsgTopicConfigureIn {
        return {
            topic: object['topic'],
            partitions: object['partitions'],
            };
    },

    _toJsonObject(self: MsgTopicConfigureIn): any {
        return {
            'topic': self.topic,
            'partitions': self.partitions,
            };
    }
}