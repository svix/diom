// this file is @generated





export interface PublishOutMsg {
    offset: number;
partition: number;
}

export const PublishOutMsgSerializer = {
    _fromJsonObject(object: any): PublishOutMsg {
        return {
            offset: object['offset'],
            partition: object['partition'],
            };
    },

    _toJsonObject(self: PublishOutMsg): any {
        return {
            'offset': self.offset,
            'partition': self.partition,
            };
    }
}