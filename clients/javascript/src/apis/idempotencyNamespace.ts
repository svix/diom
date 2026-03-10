// this file is @generated

import {
    type IdempotencyCreateNamespaceIn,
    IdempotencyCreateNamespaceInSerializer,
} from '../models/idempotencyCreateNamespaceIn';
import {
    type IdempotencyCreateNamespaceOut,
    IdempotencyCreateNamespaceOutSerializer,
} from '../models/idempotencyCreateNamespaceOut';
import {
    type IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceInSerializer,
} from '../models/idempotencyGetNamespaceIn';
import {
    type IdempotencyGetNamespaceOut,
    IdempotencyGetNamespaceOutSerializer,
} from '../models/idempotencyGetNamespaceOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class IdempotencyNamespace {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Create idempotency namespace */
    public create(
        idempotencyCreateNamespaceIn: IdempotencyCreateNamespaceIn,
        ): Promise<IdempotencyCreateNamespaceOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/idempotency/namespace/create");

        request.setBody(
            IdempotencyCreateNamespaceInSerializer._toJsonObject(
                idempotencyCreateNamespaceIn,
            )
        );
        return request.send(
            this.requestCtx,
            IdempotencyCreateNamespaceOutSerializer._fromJsonObject,
        );
    }/** Get idempotency namespace */
    public get(
        idempotencyGetNamespaceIn: IdempotencyGetNamespaceIn,
        ): Promise<IdempotencyGetNamespaceOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/idempotency/namespace/get");

        request.setBody(
            IdempotencyGetNamespaceInSerializer._toJsonObject(
                idempotencyGetNamespaceIn,
            )
        );
        return request.send(
            this.requestCtx,
            IdempotencyGetNamespaceOutSerializer._fromJsonObject,
        );
    }
}

