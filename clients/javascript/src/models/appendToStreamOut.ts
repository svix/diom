// this file is @generated





export interface AppendToStreamOut {
    msgIds: number[];
}

export const AppendToStreamOutSerializer = {
    _fromJsonObject(object: any): AppendToStreamOut {
        return {
            msgIds: object['msg_ids'],
            };
    },

    _toJsonObject(self: AppendToStreamOut): any {
        return {
            'msg_ids': self.msgIds,
            };
    }
}