// this file is @generated





export interface AckMsgRangeIn {
    consumerGroup: string;
maxMsgId: number;
minMsgId?: number | null;
name: string;
}

export const AckMsgRangeInSerializer = {
    _fromJsonObject(object: any): AckMsgRangeIn {
        return {
            consumerGroup: object['consumer_group'],
            maxMsgId: object['max_msg_id'],
            minMsgId: object['min_msg_id'],
            name: object['name'],
            };
    },

    _toJsonObject(self: AckMsgRangeIn): any {
        return {
            'consumer_group': self.consumerGroup,
            'max_msg_id': self.maxMsgId,
            'min_msg_id': self.minMsgId,
            'name': self.name,
            };
    }
}