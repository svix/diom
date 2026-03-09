// this file is @generated

export interface MsgNamespaceGetIn {
    name: string;
}

export const MsgNamespaceGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceGetIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceGetIn): any {
        return {
            'name': self.name,
        };
    }
}