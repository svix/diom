// this file is @generated

import {
    type KvDeleteIn,
    KvDeleteInSerializer,
} from '../models/kvDeleteIn';
import {
    type KvDeleteOut,
    KvDeleteOutSerializer,
} from '../models/kvDeleteOut';
import {
    type KvGetGroupIn,
    KvGetGroupInSerializer,
} from '../models/kvGetGroupIn';
import {
    type KvGetGroupOut,
    KvGetGroupOutSerializer,
} from '../models/kvGetGroupOut';
import {
    type KvGetIn,
    KvGetInSerializer,
} from '../models/kvGetIn';
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
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class Kv {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** KV Set */
        public set(
            kvSetIn: KvSetIn,
            ): Promise<KvSetOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/kv/set");

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
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/kv/get");

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

        

    /** Get KV store */
        public getGroup(
            kvGetGroupIn: KvGetGroupIn,
            ): Promise<KvGetGroupOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/kv/get-group");

            request.setBody(
                    KvGetGroupInSerializer._toJsonObject(
                        kvGetGroupIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    KvGetGroupOutSerializer._fromJsonObject,
                );
            }

        

    /** KV Delete */
        public delete(
            kvDeleteIn: KvDeleteIn,
            ): Promise<KvDeleteOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/kv/delete");

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

