// this file is @generated

import {
    type KvCreateNamespaceIn,
    KvCreateNamespaceInSerializer,
} from '../models/kvCreateNamespaceIn';
import {
    type KvCreateNamespaceOut,
    KvCreateNamespaceOutSerializer,
} from '../models/kvCreateNamespaceOut';
import {
    type KvDeleteIn,
    KvDeleteInSerializer,
} from '../models/kvDeleteIn';
import {
    type KvDeleteOut,
    KvDeleteOutSerializer,
} from '../models/kvDeleteOut';
import {
    type KvGetIn,
    KvGetInSerializer,
} from '../models/kvGetIn';
import {
    type KvGetNamespaceIn,
    KvGetNamespaceInSerializer,
} from '../models/kvGetNamespaceIn';
import {
    type KvGetNamespaceOut,
    KvGetNamespaceOutSerializer,
} from '../models/kvGetNamespaceOut';
import {
    type KvGetOut,
    KvGetOutSerializer,
} from '../models/kvGetOut';
import {
    type KvSetIn,
    KvSetInSerializer,
} from '../models/kvSetIn';
import {
    type KvSetOut,
    KvSetOutSerializer,
} from '../models/kvSetOut';
import { KvNamespace } from './kvNamespace';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Kv {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new KvNamespace(this.requestCtx);
    }

    /** KV Set */
        public set(
            kvSetIn: KvSetIn,
            ): Promise<KvSetOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/kv/set");

            request.setBody(
                    KvSetInSerializer._toJsonObject(
                        kvSetIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    KvSetOutSerializer._fromJsonObject,
                );
            }

        

    /** KV Get */
        public get(
            kvGetIn: KvGetIn,
            ): Promise<KvGetOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/kv/get");

            request.setBody(
                    KvGetInSerializer._toJsonObject(
                        kvGetIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    KvGetOutSerializer._fromJsonObject,
                );
            }

        

    /** KV Delete */
        public delete(
            kvDeleteIn: KvDeleteIn,
            ): Promise<KvDeleteOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/kv/delete");

            request.setBody(
                    KvDeleteInSerializer._toJsonObject(
                        kvDeleteIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    KvDeleteOutSerializer._fromJsonObject,
                );
            }

        

    }

