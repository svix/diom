// this file is @generated

import {
    type RateLimitCreateNamespaceIn,
    RateLimitCreateNamespaceInSerializer,
} from '../models/rateLimitCreateNamespaceIn';
import {
    type RateLimitCreateNamespaceOut,
    RateLimitCreateNamespaceOutSerializer,
} from '../models/rateLimitCreateNamespaceOut';
import {
    type RateLimitGetNamespaceIn,
    RateLimitGetNamespaceInSerializer,
} from '../models/rateLimitGetNamespaceIn';
import {
    type RateLimitGetNamespaceOut,
    RateLimitGetNamespaceOutSerializer,
} from '../models/rateLimitGetNamespaceOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class RateLimitNamespace {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Create rate limiter namespace */
        public create(
            rateLimitCreateNamespaceIn: RateLimitCreateNamespaceIn,
            ): Promise<RateLimitCreateNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limit/namespace/create");

            request.setBody(
                    RateLimitCreateNamespaceInSerializer._toJsonObject(
                        rateLimitCreateNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RateLimitCreateNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    /** Get rate limiter namespace */
        public get(
            rateLimitGetNamespaceIn: RateLimitGetNamespaceIn,
            ): Promise<RateLimitGetNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/rate-limit/namespace/get");

            request.setBody(
                    RateLimitGetNamespaceInSerializer._toJsonObject(
                        rateLimitGetNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    RateLimitGetNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    }

