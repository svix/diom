// this file is @generated

export enum RateLimitStatus {
    Ok = 'ok',
    Block = 'block',
    }

export const RateLimitStatusSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitStatus {
        return object;
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitStatus): any {
        return self;
    }
}