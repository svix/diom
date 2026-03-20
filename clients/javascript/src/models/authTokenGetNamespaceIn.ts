// this file is @generated

export interface AuthTokenGetNamespaceIn {
    name: string;
}

export const AuthTokenGetNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}