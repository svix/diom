// this file is @generated

import {
    type CacheDeleteIn,
    CacheDeleteInSerializer,
} from '../models/cacheDeleteIn';
import {
    type CacheDeleteOut,
    CacheDeleteOutSerializer,
} from '../models/cacheDeleteOut';
import {
    type CacheGetGroupIn,
    CacheGetGroupInSerializer,
} from '../models/cacheGetGroupIn';
import {
    type CacheGetGroupOut,
    CacheGetGroupOutSerializer,
} from '../models/cacheGetGroupOut';
import {
    type CacheGetIn,
    CacheGetInSerializer,
} from '../models/cacheGetIn';
import {
    type CacheGetOut,
    CacheGetOutSerializer,
} from '../models/cacheGetOut';
import {
    type CacheSetIn,
    CacheSetInSerializer,
} from '../models/cacheSetIn';
import {
    type CacheSetOut,
    CacheSetOutSerializer,
} from '../models/cacheSetOut';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export interface CacheSetOptions {
        idempotencyKey?: string;
        }

    export interface CacheGetOptions {
        idempotencyKey?: string;
        }

    export interface CacheGetGroupOptions {
        idempotencyKey?: string;
        }

    export interface CacheDeleteOptions {
        idempotencyKey?: string;
        }

    export class Cache {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Cache Set */
        public set(
            cacheSetIn: CacheSetIn,
            options?: CacheSetOptions,
            ): Promise<CacheSetOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/set");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    CacheSetInSerializer._toJsonObject(
                        cacheSetIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CacheSetOutSerializer._fromJsonObject,
                );
            }

        

    /** Cache Get */
        public get(
            cacheGetIn: CacheGetIn,
            options?: CacheGetOptions,
            ): Promise<CacheGetOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/get");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    CacheGetInSerializer._toJsonObject(
                        cacheGetIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CacheGetOutSerializer._fromJsonObject,
                );
            }

        

    /** Get cache group */
        public getGroup(
            cacheGetGroupIn: CacheGetGroupIn,
            options?: CacheGetGroupOptions,
            ): Promise<CacheGetGroupOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/get-group");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    CacheGetGroupInSerializer._toJsonObject(
                        cacheGetGroupIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CacheGetGroupOutSerializer._fromJsonObject,
                );
            }

        

    /** Cache Delete */
        public delete(
            cacheDeleteIn: CacheDeleteIn,
            options?: CacheDeleteOptions,
            ): Promise<CacheDeleteOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/delete");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
            request.setBody(
                    CacheDeleteInSerializer._toJsonObject(
                        cacheDeleteIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CacheDeleteOutSerializer._fromJsonObject,
                );
            }

        

    }

