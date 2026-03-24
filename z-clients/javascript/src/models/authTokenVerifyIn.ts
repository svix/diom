// this file is @generated

export interface AuthTokenVerifyIn {
    namespace?: string | null;
    token: string;
}

export const AuthTokenVerifyInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenVerifyIn {
        return {
            namespace: object['namespace'],
            token: object['token'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenVerifyIn): any {
        return {
            'namespace': self.namespace,
            'token': self.token,
        };
    }
}