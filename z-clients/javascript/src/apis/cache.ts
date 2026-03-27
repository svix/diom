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
import { CacheNamespace } from './cacheNamespace';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Cache {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new CacheNamespace(this.requestCtx);
    }

    /** Cache Set */
    public set(
        key: string,
        cacheSetIn: CacheSetIn,
    ): Promise<CacheSetOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cache.set");

        request.setBody(
            CacheSetInSerializer._toJsonObject({
                ...cacheSetIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            CacheSetOutSerializer._fromJsonObject,
        );
    }/** Cache Get */
    public get(
        key: string,
        cacheGetIn: CacheGetIn,
    ): Promise<CacheGetOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cache.get");

        request.setBody(
            CacheGetInSerializer._toJsonObject({
                ...cacheGetIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            CacheGetOutSerializer._fromJsonObject,
        );
    }/** Cache Delete */
    public delete(
        key: string,
        cacheDeleteIn: CacheDeleteIn,
    ): Promise<CacheDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.cache.delete");

        request.setBody(
            CacheDeleteInSerializer._toJsonObject({
                ...cacheDeleteIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            CacheDeleteOutSerializer._fromJsonObject,
        );
    }
}

