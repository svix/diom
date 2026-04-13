// this file is @generated

export interface RateLimitCreateNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const RateLimitCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCreateNamespaceOut {
        return {
            name: object['name'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCreateNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}