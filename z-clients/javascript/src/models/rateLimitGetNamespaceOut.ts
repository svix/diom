// this file is @generated

export interface RateLimitGetNamespaceOut {
    name: string;
    created: number;
    updated: number;
}

export const RateLimitGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetNamespaceOut {
        return {
            name: object['name'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created,
            'updated': self.updated,
        };
    }
}