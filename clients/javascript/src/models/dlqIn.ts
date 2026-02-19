// this file is @generated





export interface DlqIn {
    consumerGroup: string;
msgId: number;
name: string;
}

export const DlqInSerializer = {
    _fromJsonObject(object: any): DlqIn {
        return {
            consumerGroup: object['consumer_group'],
            msgId: object['msg_id'],
            name: object['name'],
            };
    },

    _toJsonObject(self: DlqIn): any {
        return {
            'consumer_group': self.consumerGroup,
            'msg_id': self.msgId,
            'name': self.name,
            };
    }
}