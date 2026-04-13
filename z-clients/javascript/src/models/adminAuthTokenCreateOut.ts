// this file is @generated

export interface AdminAuthTokenCreateOut {
    id: string;
    token: string;
    created: Date;
    updated: Date;
}

export const AdminAuthTokenCreateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenCreateOut {
        return {
            id: object['id'],
            token: object['token'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenCreateOut): any {
        return {
            'id': self.id,
            'token': self.token,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}