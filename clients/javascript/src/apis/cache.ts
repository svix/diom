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
    type CacheGetIn,
    CacheGetInSerializer,
} from '../models/cacheGetIn';
import {
    type CacheGetNamespaceIn,
    CacheGetNamespaceInSerializer,
} from '../models/cacheGetNamespaceIn';
import {
    type CacheGetNamespaceOut,
    CacheGetNamespaceOutSerializer,
} from '../models/cacheGetNamespaceOut';
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

export class Cache {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Cache Set */
        public set(
            cacheSetIn: CacheSetIn,
            ): Promise<CacheSetOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/set");

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
            ): Promise<CacheGetOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/get");

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

        

    /** Get cache namespace */
        public getNamespace(
            cacheGetNamespaceIn: CacheGetNamespaceIn,
            ): Promise<CacheGetNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/get-namespace");

            request.setBody(
                    CacheGetNamespaceInSerializer._toJsonObject(
                        cacheGetNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    CacheGetNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    /** Cache Delete */
        public delete(
            cacheDeleteIn: CacheDeleteIn,
            ): Promise<CacheDeleteOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/cache/delete");

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

