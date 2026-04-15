// this file is @generated

import {
    type KvConfigureNamespaceIn,
    KvConfigureNamespaceInSerializer,
} from '../models/kvConfigureNamespaceIn';
import {
    type KvConfigureNamespaceOut,
    KvConfigureNamespaceOutSerializer,
} from '../models/kvConfigureNamespaceOut';
import {
    type KvGetNamespaceIn,
    KvGetNamespaceInSerializer,
} from '../models/kvGetNamespaceIn';
import {
    type KvGetNamespaceOut,
    KvGetNamespaceOutSerializer,
} from '../models/kvGetNamespaceOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class KvNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Configure KV namespace */
    public configure(
        kvConfigureNamespaceIn: KvConfigureNamespaceIn,
    ): Promise<KvConfigureNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.kv.namespace.configure");

        request.setBody(
            KvConfigureNamespaceInSerializer._toJsonObject(kvConfigureNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            KvConfigureNamespaceOutSerializer._fromJsonObject,
        );
    }/** Get KV namespace */
    public get(
        kvGetNamespaceIn: KvGetNamespaceIn,
    ): Promise<KvGetNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.kv.namespace.get");

        request.setBody(
            KvGetNamespaceInSerializer._toJsonObject(kvGetNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            KvGetNamespaceOutSerializer._fromJsonObject,
        );
    }
}

