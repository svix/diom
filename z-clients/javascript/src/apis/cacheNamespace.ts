// this file is @generated

import {
    type CacheConfigureNamespaceIn,
    CacheConfigureNamespaceInSerializer,
} from '../models/cacheConfigureNamespaceIn';
import {
    type CacheConfigureNamespaceOut,
    CacheConfigureNamespaceOutSerializer,
} from '../models/cacheConfigureNamespaceOut';
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

    /** Configure cache namespace */
    public configure(
        cacheConfigureNamespaceIn: CacheConfigureNamespaceIn,
    ): Promise<CacheConfigureNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cache.namespace.configure");

        request.setBody(
            CacheConfigureNamespaceInSerializer._toJsonObject(cacheConfigureNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            CacheConfigureNamespaceOutSerializer._fromJsonObject,
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

