// this file is @generated

import {
    type IdempotencyConfigureNamespaceIn,
    IdempotencyConfigureNamespaceInSerializer,
} from '../models/idempotencyConfigureNamespaceIn';
import {
    type IdempotencyConfigureNamespaceOut,
    IdempotencyConfigureNamespaceOutSerializer,
} from '../models/idempotencyConfigureNamespaceOut';
import {
    type IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceInSerializer,
} from '../models/idempotencyGetNamespaceIn';
import {
    type IdempotencyGetNamespaceOut,
    IdempotencyGetNamespaceOutSerializer,
} from '../models/idempotencyGetNamespaceOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class IdempotencyNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Configure idempotency namespace */
    public configure(
        idempotencyConfigureNamespaceIn: IdempotencyConfigureNamespaceIn,
    ): Promise<IdempotencyConfigureNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.idempotency.namespace.configure");

        request.setBody(
            IdempotencyConfigureNamespaceInSerializer._toJsonObject(idempotencyConfigureNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            IdempotencyConfigureNamespaceOutSerializer._fromJsonObject,
        );
    }/** Get idempotency namespace */
    public get(
        idempotencyGetNamespaceIn: IdempotencyGetNamespaceIn,
    ): Promise<IdempotencyGetNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.idempotency.namespace.get");

        request.setBody(
            IdempotencyGetNamespaceInSerializer._toJsonObject(idempotencyGetNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            IdempotencyGetNamespaceOutSerializer._fromJsonObject,
        );
    }
}

