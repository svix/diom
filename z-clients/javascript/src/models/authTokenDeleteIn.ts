// this file is @generated

export interface AuthTokenDeleteIn {
    namespace?: string | null;
    id: string;
}

export const AuthTokenDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenDeleteIn {
        return {
            namespace: object['namespace'],
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenDeleteIn): any {
        return {
            'namespace': self.namespace,
            'id': self.id,
        };
    }
}