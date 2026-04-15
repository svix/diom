// this file is @generated

import {
    type RateLimitConfigureNamespaceIn,
    RateLimitConfigureNamespaceInSerializer,
} from '../models/rateLimitConfigureNamespaceIn';
import {
    type RateLimitConfigureNamespaceOut,
    RateLimitConfigureNamespaceOutSerializer,
} from '../models/rateLimitConfigureNamespaceOut';
import {
    type RateLimitGetNamespaceIn,
    RateLimitGetNamespaceInSerializer,
} from '../models/rateLimitGetNamespaceIn';
import {
    type RateLimitGetNamespaceOut,
    RateLimitGetNamespaceOutSerializer,
} from '../models/rateLimitGetNamespaceOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class RateLimitNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Configure rate limiter namespace */
    public configure(
        rateLimitConfigureNamespaceIn: RateLimitConfigureNamespaceIn,
    ): Promise<RateLimitConfigureNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.rate-limit.namespace.configure");

        request.setBody(
            RateLimitConfigureNamespaceInSerializer._toJsonObject(rateLimitConfigureNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            RateLimitConfigureNamespaceOutSerializer._fromJsonObject,
        );
    }/** Get rate limiter namespace */
    public get(
        rateLimitGetNamespaceIn: RateLimitGetNamespaceIn,
    ): Promise<RateLimitGetNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.rate-limit.namespace.get");

        request.setBody(
            RateLimitGetNamespaceInSerializer._toJsonObject(rateLimitGetNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            RateLimitGetNamespaceOutSerializer._fromJsonObject,
        );
    }
}

