// this file is @generated





export interface MsgPublishOutMsg {
    topic: string;
offset: number;
}

export const MsgPublishOutMsgSerializer = {
    _fromJsonObject(object: any): MsgPublishOutMsg {
        return {
            topic: object['topic'],
            offset: object['offset'],
            };
    },

    _toJsonObject(self: MsgPublishOutMsg): any {
        return {
            'topic': self.topic,
            'offset': self.offset,
            };
    }
}