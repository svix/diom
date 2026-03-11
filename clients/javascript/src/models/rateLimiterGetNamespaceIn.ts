// this file is @generated

export interface RateLimiterGetNamespaceIn {
    name: string;
}

export const RateLimiterGetNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimiterGetNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimiterGetNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}