// this file is @generated





export interface KvGetNamespaceIn {
    name: string;
}

export const KvGetNamespaceInSerializer = {
    _fromJsonObject(object: any): KvGetNamespaceIn {
        return {
            name: object['name'],
            };
    },

    _toJsonObject(self: KvGetNamespaceIn): any {
        return {
            'name': self.name,
            };
    }
}