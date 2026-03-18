// this file is @generated

export interface AuthTokenCreateOut {
    id: string;
    createdAt: Date;
    updatedAt: Date;
    token: string;
}

export const AuthTokenCreateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateOut {
        return {
            id: object['id'],
            createdAt: new Date(object['created_at']),
            updatedAt: new Date(object['updated_at']),
            token: object['token'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateOut): any {
        return {
            'id': self.id,
            'created_at': self.createdAt,
            'updated_at': self.updatedAt,
            'token': self.token,
        };
    }
}