// this file is @generated

export interface RateLimitGetNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const RateLimitGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetNamespaceOut {
        return {
            name: object['name'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}