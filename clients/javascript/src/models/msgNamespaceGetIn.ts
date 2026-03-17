// this file is @generated
// biome-ignore-all lint/suspicious/noEmptyInterface: forwards compat

export interface MsgNamespaceGetIn {
}

export interface MsgNamespaceGetIn_ {
    name: string;
}

export const MsgNamespaceGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceGetIn_ {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceGetIn_): any {
        return {
            'name': self.name,
        };
    }
}