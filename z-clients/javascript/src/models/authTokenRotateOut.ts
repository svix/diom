// this file is @generated

export interface AuthTokenRotateOut {
    id: string;
    created: Date;
    updated: Date;
    token: string;
}

export const AuthTokenRotateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenRotateOut {
        return {
            id: object['id'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
            token: object['token'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenRotateOut): any {
        return {
            'id': self.id,
            'created': self.created,
            'updated': self.updated,
            'token': self.token,
        };
    }
}