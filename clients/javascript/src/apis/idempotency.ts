// this file is @generated

import {
    type IdempotencyAbortIn,
    IdempotencyAbortInSerializer,
} from '../models/idempotencyAbortIn';
import {
    type IdempotencyAbortOut,
    IdempotencyAbortOutSerializer,
} from '../models/idempotencyAbortOut';
import { IdempotencyNamespace } from './idempotencyNamespace';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class Idempotency {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get namespace() {
        return new IdempotencyNamespace(this.requestCtx);
    }

    /** Abandon an idempotent request (remove lock without saving response) */
    public abort(
        idempotencyAbortIn: IdempotencyAbortIn,
        ): Promise<IdempotencyAbortOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/idempotency/abort");

        request.setBody(
            IdempotencyAbortInSerializer._toJsonObject(
                idempotencyAbortIn,
            )
        );
        return request.send(
            this.requestCtx,
            IdempotencyAbortOutSerializer._fromJsonObject,
        );
    }
}

