// this file is @generated





export interface TopicConfigureOut {
    partitions: number;
}

export const TopicConfigureOutSerializer = {
    _fromJsonObject(object: any): TopicConfigureOut {
        return {
            partitions: object['partitions'],
            };
    },

    _toJsonObject(self: TopicConfigureOut): any {
        return {
            'partitions': self.partitions,
            };
    }
}