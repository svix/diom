// this file is @generated

export interface RateLimitConfigureNamespaceIn {
    name: string;
}

export const RateLimitConfigureNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitConfigureNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitConfigureNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}