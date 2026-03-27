// this file is @generated

export interface AdminAuthTokenListIn {
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const AdminAuthTokenListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenListIn {
        return {
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenListIn): any {
        return {
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}