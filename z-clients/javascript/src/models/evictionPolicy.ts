// this file is @generated

export enum EvictionPolicy {
    NoEviction = 'NoEviction',
    LeastRecentlyUsed = 'LeastRecentlyUsed',
    }

export const EvictionPolicySerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): EvictionPolicy {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: EvictionPolicy): any {
        return self;
    }
}