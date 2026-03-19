// this file is @generated
import {
    type RateLimitTokenBucketConfig,
    RateLimitTokenBucketConfigSerializer,
} from './rateLimitTokenBucketConfig';

export interface RateLimitGetRemainingIn {
    namespace?: string | null;
    key: string;
    /** Rate limiter configuration */
    config: RateLimitTokenBucketConfig;
}

export const RateLimitGetRemainingInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingIn {
        return {
            namespace: object['namespace'],
            key: object['key'],
            config: RateLimitTokenBucketConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingIn): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'config': RateLimitTokenBucketConfigSerializer._toJsonObject(self.config),
        };
    }
}