// this file is @generated





export interface MsgPublishOutTopic {
    topic: string;
    startOffset: number;
    offset: number;
}

export const MsgPublishOutTopicSerializer = {
    _fromJsonObject(object: any): MsgPublishOutTopic {
        return {
            topic: object['topic'],
            startOffset: object['start_offset'],
            offset: object['offset'],
            };
    },

    _toJsonObject(self: MsgPublishOutTopic): any {
        return {
            'topic': self.topic,
            'start_offset': self.startOffset,
            'offset': self.offset,
            };
    }
}