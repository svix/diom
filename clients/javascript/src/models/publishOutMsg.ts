// this file is @generated





export interface PublishOutMsg {
    offset: number;
topic: string;
}

export const PublishOutMsgSerializer = {
    _fromJsonObject(object: any): PublishOutMsg {
        return {
            offset: object['offset'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: PublishOutMsg): any {
        return {
            'offset': self.offset,
            'topic': self.topic,
            };
    }
}