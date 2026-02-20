// this file is @generated





export interface RateLimiterFixedWindowConfig {
    /** Maximum number of requests allowed within the window */
    maxRequests: number;
/** Window size in seconds */
    windowSize: number;
}

export const RateLimiterFixedWindowConfigSerializer = {
    _fromJsonObject(object: any): RateLimiterFixedWindowConfig {
        return {
            maxRequests: object['max_requests'],
            windowSize: object['window_size'],
            };
    },

    _toJsonObject(self: RateLimiterFixedWindowConfig): any {
        return {
            'max_requests': self.maxRequests,
            'window_size': self.windowSize,
            };
    }
}