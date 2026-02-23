// this file is @generated





export interface GetStreamIn {
    name: string;
}

export const GetStreamInSerializer = {
    _fromJsonObject(object: any): GetStreamIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: GetStreamIn): any {
        return {
            'name': self.name,
            };
    }
}