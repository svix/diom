// this file is @generated

export interface AuthTokenListIn {
    namespace?: string | null;
    ownerId: string;
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const AuthTokenListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenListIn {
        return {
            namespace: object['namespace'],
            ownerId: object['owner_id'],
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenListIn): any {
        return {
            'namespace': self.namespace,
            'owner_id': self.ownerId,
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}