// this file is @generated

export interface MsgNamespaceGetIn {
    name: string;
}

export const MsgNamespaceGetInSerializer = {
    _fromJsonObject(object: any): MsgNamespaceGetIn {
        return {
            name: object['name'],
        };
    },

    _toJsonObject(self: MsgNamespaceGetIn): any {
        return {
            'name': self.name,
        };
    }
}