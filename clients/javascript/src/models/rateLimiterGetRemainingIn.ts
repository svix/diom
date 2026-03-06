// this file is @generated
import {
    type RateLimiterFixedWindowConfig,
    RateLimiterFixedWindowConfigSerializer,
} from './rateLimiterFixedWindowConfig';
import {
    type RateLimiterTokenBucketConfig,
    RateLimiterTokenBucketConfigSerializer,
} from './rateLimiterTokenBucketConfig';
interface _RateLimiterGetRemainingInFields {
    key: string;}


    

    



interface RateLimiterGetRemainingInTokenBucket {
    method: 'token_bucket';
    config: RateLimiterTokenBucketConfig;
    
}

interface RateLimiterGetRemainingInFixedWindow {
    method: 'fixed_window';
    config: RateLimiterFixedWindowConfig;
    
}



export type RateLimiterGetRemainingIn = _RateLimiterGetRemainingInFields & (| RateLimiterGetRemainingInTokenBucket
    | RateLimiterGetRemainingInFixedWindow
    );

export const RateLimiterGetRemainingInSerializer = {
    _fromJsonObject(object: any): RateLimiterGetRemainingIn {
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
            };
    },

    _toJsonObject(self: RateLimiterGetRemainingIn): any {
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
            };
    }
}