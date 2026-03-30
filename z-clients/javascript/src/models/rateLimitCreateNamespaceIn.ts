// this file is @generated

export interface RateLimitCreateNamespaceIn {
    name: string;
}

export const RateLimitCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCreateNamespaceIn {
        return {
            name: object['name'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCreateNamespaceIn): any {
        return {
            'name': self.name,
        };
    }
}