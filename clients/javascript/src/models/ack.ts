// this file is @generated





export interface Ack {
    consumerGroup: string;
msgId: number;
name: string;
}

export const AckSerializer = {
    _fromJsonObject(object: any): Ack {
        return {
            consumerGroup: object['consumer_group'],
            msgId: object['msg_id'],
            name: object['name'],
            };
    },

    _toJsonObject(self: Ack): any {
        return {
            'consumer_group': self.consumerGroup,
            'msg_id': self.msgId,
            'name': self.name,
            };
    }
}