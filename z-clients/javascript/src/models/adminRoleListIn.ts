// this file is @generated

export interface AdminRoleListIn {
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const AdminRoleListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleListIn {
        return {
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleListIn): any {
        return {
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}