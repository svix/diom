// this file is @generated

import {
    type RateLimiterCheckIn,
    RateLimiterCheckInSerializer,
} from '../models/rateLimiterCheckIn';
import {
    type RateLimiterCheckOut,
    RateLimiterCheckOutSerializer,
} from '../models/rateLimiterCheckOut';
import {
    type RateLimiterGetRemainingIn,
    RateLimiterGetRemainingInSerializer,
} from '../models/rateLimiterGetRemainingIn';
import {
    type RateLimiterGetRemainingOut,
    RateLimiterGetRemainingOutSerializer,
} from '../models/rateLimiterGetRemainingOut';
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export interface RateLimiterLimitOptions {
        idempotencyKey?: string;
        }

    export interface RateLimiterGetRemainingOptions {
        idempotencyKey?: string;
        }

    export class RateLimiter {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Rate Limiter Check and Consume */
        public limit(
            rateLimiterCheckIn: RateLimiterCheckIn,
            options?: RateLimiterLimitOptions,
            ): Promise<RateLimiterCheckOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/rate-limiter/limit");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    RateLimiterCheckInSerializer._toJsonObject(
                        rateLimiterCheckIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RateLimiterCheckOutSerializer._fromJsonObject,
                );
            }

        

    /** Rate Limiter Get Remaining */
        public getRemaining(
            rateLimiterGetRemainingIn: RateLimiterGetRemainingIn,
            options?: RateLimiterGetRemainingOptions,
            ): Promise<RateLimiterGetRemainingOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/rate-limiter/get-remaining");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    RateLimiterGetRemainingInSerializer._toJsonObject(
                        rateLimiterGetRemainingIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RateLimiterGetRemainingOutSerializer._fromJsonObject,
                );
            }

        

    }

