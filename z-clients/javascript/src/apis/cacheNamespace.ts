// this file is @generated

import {
    type CacheCreateNamespaceIn,
    CacheCreateNamespaceInSerializer,
} from '../models/cacheCreateNamespaceIn';
import {
    type CacheCreateNamespaceOut,
    CacheCreateNamespaceOutSerializer,
} from '../models/cacheCreateNamespaceOut';
import {
    type CacheGetNamespaceIn,
    CacheGetNamespaceInSerializer,
} from '../models/cacheGetNamespaceIn';
import {
    type CacheGetNamespaceOut,
    CacheGetNamespaceOutSerializer,
} from '../models/cacheGetNamespaceOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class CacheNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create cache namespace */
    public create(
        cacheCreateNamespaceIn: CacheCreateNamespaceIn,
    ): Promise<CacheCreateNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cache.namespace.create");

        request.setBody(
            CacheCreateNamespaceInSerializer._toJsonObject(cacheCreateNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            CacheCreateNamespaceOutSerializer._fromJsonObject,
        );
    }/** Get cache namespace */
    public get(
        cacheGetNamespaceIn: CacheGetNamespaceIn,
    ): Promise<CacheGetNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cache.namespace.get");

        request.setBody(
            CacheGetNamespaceInSerializer._toJsonObject(cacheGetNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            CacheGetNamespaceOutSerializer._fromJsonObject,
        );
    }
}

