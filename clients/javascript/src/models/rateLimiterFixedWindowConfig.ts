// this file is @generated

export interface RateLimiterFixedWindowConfig {
    /** Window size in seconds */
    windowSize: number;
    /** Maximum number of requests allowed within the window */
    maxRequests: number;
}

export const RateLimiterFixedWindowConfigSerializer = {
    _fromJsonObject(object: any): RateLimiterFixedWindowConfig {
        return {
            windowSize: object['window_size'],
            maxRequests: object['max_requests'],
            };
    },

    _toJsonObject(self: RateLimiterFixedWindowConfig): any {
        return {
            'window_size': self.windowSize,
            'max_requests': self.maxRequests,
            };
    }
}