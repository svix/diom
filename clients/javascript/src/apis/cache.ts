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
import { CacheNamespace } from './cacheNamespace';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class Cache {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get namespace() {
        return new CacheNamespace(this.requestCtx);
    }

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

