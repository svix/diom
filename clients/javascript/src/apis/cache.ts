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
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class Cache {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Cache Set */
        public set(
            cacheSetIn: CacheSetIn,
            ): Promise<CacheSetOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/cache/set");

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
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/cache/get");

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
            ): Promise<CacheGetGroupOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/cache/get-group");

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
            ): Promise<CacheDeleteOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/cache/delete");

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

