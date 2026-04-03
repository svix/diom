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
import { KvNamespace } from './kvNamespace';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Kv {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new KvNamespace(this.requestCtx);
    }

    /** KV Set */
    public set(
        key: string,
        value: number[],
        kvSetIn: KvSetIn,
    ): Promise<KvSetOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.kv.set");

        request.setBody(
            KvSetInSerializer._toJsonObject({
                ...kvSetIn,
                key: key,
                value: value,
            })
        );
        
        return request.send(
            this.requestCtx,
            KvSetOutSerializer._fromJsonObject,
        );
    }/** KV Get */
    public get(
        key: string,
        kvGetIn: KvGetIn,
    ): Promise<KvGetOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.kv.get");

        request.setBody(
            KvGetInSerializer._toJsonObject({
                ...kvGetIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            KvGetOutSerializer._fromJsonObject,
        );
    }/** KV Delete */
    public delete(
        key: string,
        kvDeleteIn: KvDeleteIn,
    ): Promise<KvDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.kv.delete");

        request.setBody(
            KvDeleteInSerializer._toJsonObject({
                ...kvDeleteIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            KvDeleteOutSerializer._fromJsonObject,
        );
    }
}

