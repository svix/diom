// this file is @generated

export interface RateLimitConfigureNamespaceOut {
    name: string;
    created: Date;
    updated: Date;
}

export const RateLimitConfigureNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitConfigureNamespaceOut {
        return {
            name: object['name'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitConfigureNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}