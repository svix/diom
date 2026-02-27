// this file is @generated





export interface GetNamespaceIn {
    name: string;
}

export const GetNamespaceInSerializer = {
    _fromJsonObject(object: any): GetNamespaceIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: GetNamespaceIn): any {
        return {
            'name': self.name,
            };
    }
}