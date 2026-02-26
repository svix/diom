// this file is @generated

import {
    type IdempotencyAbortIn,
    IdempotencyAbortInSerializer,
} from '../models/idempotencyAbortIn';
import {
    type IdempotencyAbortOut,
    IdempotencyAbortOutSerializer,
} from '../models/idempotencyAbortOut';
import {
    type IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceInSerializer,
} from '../models/idempotencyGetNamespaceIn';
import {
    type IdempotencyGetNamespaceOut,
    IdempotencyGetNamespaceOutSerializer,
} from '../models/idempotencyGetNamespaceOut';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class Idempotency {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

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

        

    /** Get idempotency namespace */
        public getNamespace(
            idempotencyGetNamespaceIn: IdempotencyGetNamespaceIn,
            ): Promise<IdempotencyGetNamespaceOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/idempotency/get-namespace");

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

