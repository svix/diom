// this file is @generated

export interface AdminAuthTokenRotateOut {
    id: string;
    token: string;
    created: number;
    updated: number;
}

export const AdminAuthTokenRotateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenRotateOut {
        return {
            id: object['id'],
            token: object['token'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenRotateOut): any {
        return {
            'id': self.id,
            'token': self.token,
            'created': self.created,
            'updated': self.updated,
        };
    }
}