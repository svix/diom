// this file is @generated

import {
    type RateLimitCheckIn,
    RateLimitCheckInSerializer,
} from '../models/rateLimitCheckIn';
import {
    type RateLimitCheckOut,
    RateLimitCheckOutSerializer,
} from '../models/rateLimitCheckOut';
import {
    type RateLimitGetRemainingIn,
    RateLimitGetRemainingInSerializer,
} from '../models/rateLimitGetRemainingIn';
import {
    type RateLimitGetRemainingOut,
    RateLimitGetRemainingOutSerializer,
} from '../models/rateLimitGetRemainingOut';
import {
    type RateLimitResetIn,
    RateLimitResetInSerializer,
} from '../models/rateLimitResetIn';
import {
    type RateLimitResetOut,
    RateLimitResetOutSerializer,
} from '../models/rateLimitResetOut';
import { RateLimitNamespace } from './rateLimitNamespace';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class RateLimit {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get namespace() {
        return new RateLimitNamespace(this.requestCtx);
    }

    /** Rate Limiter Check and Consume */
    public limit(
        rateLimitCheckIn: RateLimitCheckIn,
        ): Promise<RateLimitCheckOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limit/limit");

        request.setBody(
            RateLimitCheckInSerializer._toJsonObject(
                rateLimitCheckIn,
            )
        );
        return request.send(
            this.requestCtx,
            RateLimitCheckOutSerializer._fromJsonObject,
        );
    }/** Rate Limiter Get Remaining */
    public getRemaining(
        rateLimitGetRemainingIn: RateLimitGetRemainingIn,
        ): Promise<RateLimitGetRemainingOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limit/get-remaining");

        request.setBody(
            RateLimitGetRemainingInSerializer._toJsonObject(
                rateLimitGetRemainingIn,
            )
        );
        return request.send(
            this.requestCtx,
            RateLimitGetRemainingOutSerializer._fromJsonObject,
        );
    }/** Rate Limiter Reset */
    public reset(
        rateLimitResetIn: RateLimitResetIn,
        ): Promise<RateLimitResetOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limit/reset");

        request.setBody(
            RateLimitResetInSerializer._toJsonObject(
                rateLimitResetIn,
            )
        );
        return request.send(
            this.requestCtx,
            RateLimitResetOutSerializer._fromJsonObject,
        );
    }
}

