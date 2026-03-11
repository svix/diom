// this file is @generated

import {
    type RateLimiterCreateNamespaceIn,
    RateLimiterCreateNamespaceInSerializer,
} from '../models/rateLimiterCreateNamespaceIn';
import {
    type RateLimiterCreateNamespaceOut,
    RateLimiterCreateNamespaceOutSerializer,
} from '../models/rateLimiterCreateNamespaceOut';
import {
    type RateLimiterGetNamespaceIn,
    RateLimiterGetNamespaceInSerializer,
} from '../models/rateLimiterGetNamespaceIn';
import {
    type RateLimiterGetNamespaceOut,
    RateLimiterGetNamespaceOutSerializer,
} from '../models/rateLimiterGetNamespaceOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class RateLimiterNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create rate limiter namespace */
        public create(
            rateLimiterCreateNamespaceIn: RateLimiterCreateNamespaceIn,
            ): Promise<RateLimiterCreateNamespaceOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/rate-limit/namespace/create");

            request.setBody(
                    RateLimiterCreateNamespaceInSerializer._toJsonObject(
                        rateLimiterCreateNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RateLimiterCreateNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    /** Get rate limiter namespace */
        public get(
            rateLimiterGetNamespaceIn: RateLimiterGetNamespaceIn,
            ): Promise<RateLimiterGetNamespaceOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/rate-limit/namespace/get");

            request.setBody(
                    RateLimiterGetNamespaceInSerializer._toJsonObject(
                        rateLimiterGetNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RateLimiterGetNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    }

