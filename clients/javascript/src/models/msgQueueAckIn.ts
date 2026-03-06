// this file is @generated

export interface MsgQueueAckIn {
    topic: string;
    msgIds: string[];
}

export const MsgQueueAckInSerializer = {
    _fromJsonObject(object: any): MsgQueueAckIn {
        return {
            topic: object['topic'],
            msgIds: object['msg_ids'],
        };
    },

    _toJsonObject(self: MsgQueueAckIn): any {
        return {
            'topic': self.topic,
            'msg_ids': self.msgIds,
        };
    }
}