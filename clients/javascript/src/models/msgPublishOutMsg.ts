// this file is @generated





export interface MsgPublishOutMsg {
    offset: number;
topic: string;
}

export const MsgPublishOutMsgSerializer = {
    _fromJsonObject(object: any): MsgPublishOutMsg {
        return {
            offset: object['offset'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: MsgPublishOutMsg): any {
        return {
            'offset': self.offset,
            'topic': self.topic,
            };
    }
}