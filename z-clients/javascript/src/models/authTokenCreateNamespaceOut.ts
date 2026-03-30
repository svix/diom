// this file is @generated

export interface AuthTokenCreateNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const AuthTokenCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateNamespaceOut {
        return {
            name: object['name'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created,
            'updated': self.updated,
        };
    }
}