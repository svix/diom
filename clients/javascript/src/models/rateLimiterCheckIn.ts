// this file is @generated
import {
    type RateLimiterFixedWindowConfig,
    RateLimiterFixedWindowConfigSerializer,
} from './rateLimiterFixedWindowConfig';
import {
    type RateLimiterTokenBucketConfig,
    RateLimiterTokenBucketConfigSerializer,
} from './rateLimiterTokenBucketConfig';
interface _RateLimiterCheckInFields {
    key: string;
    /** Number of tokens to consume (default: 1) */
    tokens?: number;}


    

    



interface RateLimiterCheckInTokenBucket {
    method: 'token_bucket';
    config: RateLimiterTokenBucketConfig;
    
}

interface RateLimiterCheckInFixedWindow {
    method: 'fixed_window';
    config: RateLimiterFixedWindowConfig;
    
}



export type RateLimiterCheckIn = _RateLimiterCheckInFields & (| RateLimiterCheckInTokenBucket
    | RateLimiterCheckInFixedWindow
    );

export const RateLimiterCheckInSerializer = {
    _fromJsonObject(object: any): RateLimiterCheckIn {
        const method = object['method'];

        function getConfig(method: string): any {
            switch (method) {
                case 'token_bucket':
                    return RateLimiterTokenBucketConfigSerializer._fromJsonObject(
                            object['config']
                        );
                case 'fixed_window':
                    return RateLimiterFixedWindowConfigSerializer._fromJsonObject(
                            object['config']
                        );default:
                    throw new Error(`Unexpected method: ${ method }`);
            }
        }

        return {
            method,
            config:getConfig(method),
            key: object['key'],
            tokens: object['tokens'],
            };
    },

    _toJsonObject(self: RateLimiterCheckIn): any {
        // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
        let config;
        switch (self.method) {
            case 'token_bucket':
                config =
                    RateLimiterTokenBucketConfigSerializer._toJsonObject(
                        self.config
                    );
                break;
            case 'fixed_window':
                config =
                    RateLimiterFixedWindowConfigSerializer._toJsonObject(
                        self.config
                    );
                break;}

        return {
            'method': self.method,
            'config': config,
            'key': self.key,
            'tokens': self.tokens,
            };
    }
}