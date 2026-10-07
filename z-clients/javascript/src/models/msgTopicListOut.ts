// this file is @generated

export interface MsgTopicListOut {
    id: string;
    name: string;
    partitions: number;
}

export const MsgTopicListOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicListOut {
        return {
            id: object['id'],
            name: object['name'],
            partitions: object['partitions'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicListOut): any {
        return {
            'id': self.id,
            'name': self.name,
            'partitions': self.partitions,
        };
    }
}