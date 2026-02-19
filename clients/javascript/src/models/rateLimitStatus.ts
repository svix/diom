// this file is @generated

export enum RateLimitStatus {
    Ok = 'ok',
    Block = 'block',
    }

export const RateLimitStatusSerializer = {
    _fromJsonObject(object: any): RateLimitStatus {
        return object;
    },

    _toJsonObject(self: RateLimitStatus): any {
        return self;
    }
}