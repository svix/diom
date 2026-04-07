// this file is @generated

export interface RateLimitCreateNamespaceOut {
    name: string;
    created: number;
    updated: number;
}

export const RateLimitCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCreateNamespaceOut {
        return {
            name: object['name'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCreateNamespaceOut): any {
        return {
            'name': self.name,
            'created': self.created,
            'updated': self.updated,
        };
    }
}