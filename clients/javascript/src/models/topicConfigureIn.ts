// this file is @generated





export interface TopicConfigureIn {
    name: string;
partitions: number;
topic: string;
}

export const TopicConfigureInSerializer = {
    _fromJsonObject(object: any): TopicConfigureIn {
        return {
            name: object['name'],
            partitions: object['partitions'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: TopicConfigureIn): any {
        return {
            'name': self.name,
            'partitions': self.partitions,
            'topic': self.topic,
            };
    }
}