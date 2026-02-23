// this file is @generated

export enum EvictionPolicy {
    NoEviction = 'NoEviction',
    LeastRecentlyUsed = 'LeastRecentlyUsed',
    }

export const EvictionPolicySerializer = {
    _fromJsonObject(object: any): EvictionPolicy {
        return object;
    },

    _toJsonObject(self: EvictionPolicy): any {
        return self;
    }
}