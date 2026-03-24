// this file is @generated

export interface AuthTokenCreateOut {
    id: string;
    created: Date;
    updated: Date;
    token: string;
}

export const AuthTokenCreateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateOut {
        return {
            id: object['id'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
            token: object['token'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateOut): any {
        return {
            'id': self.id,
            'created': self.created,
            'updated': self.updated,
            'token': self.token,
        };
    }
}