// this file is @generated





export interface KvGetGroupIn {
    name: string;
}

export const KvGetGroupInSerializer = {
    _fromJsonObject(object: any): KvGetGroupIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: KvGetGroupIn): any {
        return {
            'name': self.name,
            };
    }
}