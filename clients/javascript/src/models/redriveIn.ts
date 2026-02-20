// this file is @generated





export interface RedriveIn {
    consumerGroup: string;
name: string;
}

export const RedriveInSerializer = {
    _fromJsonObject(object: any): RedriveIn {
        return {
            consumerGroup: object['consumer_group'],
            name: object['name'],
            };
    },

    _toJsonObject(self: RedriveIn): any {
        return {
            'consumer_group': self.consumerGroup,
            'name': self.name,
            };
    }
}