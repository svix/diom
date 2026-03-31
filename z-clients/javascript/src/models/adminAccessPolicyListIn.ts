// this file is @generated

export interface AdminAccessPolicyListIn {
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const AdminAccessPolicyListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyListIn {
        return {
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyListIn): any {
        return {
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}