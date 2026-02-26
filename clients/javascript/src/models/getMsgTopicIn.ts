// this file is @generated





export interface GetMsgTopicIn {
    name: string;
}

export const GetMsgTopicInSerializer = {
    _fromJsonObject(object: any): GetMsgTopicIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: GetMsgTopicIn): any {
        return {
            'name': self.name,
            };
    }
}