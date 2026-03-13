// this file is @generated

export interface RateLimitGetNamespaceIn {
    name: string;
}

export const RateLimitGetNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}