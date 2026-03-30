// this file is @generated

export interface AuthTokenCreateNamespaceIn {
    name: string;
}

export const AuthTokenCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}