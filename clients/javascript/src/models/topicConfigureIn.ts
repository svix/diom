// this file is @generated





export interface TopicConfigureIn {
    partitions: number;
topic: string;
}

export const TopicConfigureInSerializer = {
    _fromJsonObject(object: any): TopicConfigureIn {
        return {
            partitions: object['partitions'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: TopicConfigureIn): any {
        return {
            'partitions': self.partitions,
            'topic': self.topic,
            };
    }
}