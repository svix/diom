// this file is @generated

export interface AdminAuthTokenCreateOut {
    id: string;
    token: string;
    created: number;
    updated: number;
}

export const AdminAuthTokenCreateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenCreateOut {
        return {
            id: object['id'],
            token: object['token'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenCreateOut): any {
        return {
            'id': self.id,
            'token': self.token,
            'created': self.created,
            'updated': self.updated,
        };
    }
}