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
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export interface KvSetOptions {
        idempotencyKey?: string;
        }

    export interface KvGetOptions {
        idempotencyKey?: string;
        }

    export interface KvDeleteOptions {
        idempotencyKey?: string;
        }

    export class Kv {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** KV Set */
        public set(
            kvSetIn: KvSetIn,
            options?: KvSetOptions,
            ): Promise<KvSetOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/kv/set");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
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
            options?: KvGetOptions,
            ): Promise<KvGetOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/kv/get");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
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
            options?: KvDeleteOptions,
            ): Promise<KvDeleteOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/kv/delete");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
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

