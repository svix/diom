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
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class RateLimiter {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Rate Limiter Check and Consume */
        public limit(
            rateLimiterCheckIn: RateLimiterCheckIn,
            ): Promise<RateLimiterCheckOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limiter/limit");

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
            ): Promise<RateLimiterGetRemainingOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limiter/get-remaining");

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

