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
    type KvGetNamespaceIn,
    KvGetNamespaceInSerializer,
} from '../models/kvGetNamespaceIn';
import {
    type KvGetNamespaceOut,
    KvGetNamespaceOutSerializer,
} from '../models/kvGetNamespaceOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class KvNamespace {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Create KV namespace */
        public create(
            kvCreateNamespaceIn: KvCreateNamespaceIn,
            ): Promise<KvCreateNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/kv/namespace/create");

            request.setBody(
                    KvCreateNamespaceInSerializer._toJsonObject(
                        kvCreateNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    KvCreateNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    /** Get KV namespace */
        public get(
            kvGetNamespaceIn: KvGetNamespaceIn,
            ): Promise<KvGetNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/kv/namespace/get");

            request.setBody(
                    KvGetNamespaceInSerializer._toJsonObject(
                        kvGetNamespaceIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    KvGetNamespaceOutSerializer._fromJsonObject,
                );
            }

        

    }

