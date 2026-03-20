// this file is @generated

export interface AuthTokenListIn {
    namespace?: string | null;
    ownerId: string;
}

export const AuthTokenListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenListIn {
        return {
            namespace: object['namespace'],
            ownerId: object['owner_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenListIn): any {
        return {
            'namespace': self.namespace,
            'owner_id': self.ownerId,
        };
    }
}