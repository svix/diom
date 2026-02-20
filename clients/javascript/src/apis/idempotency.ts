// this file is @generated

import {
    type IdempotencyAbortIn,
    IdempotencyAbortInSerializer,
} from '../models/idempotencyAbortIn';
import {
    type IdempotencyAbortOut,
    IdempotencyAbortOutSerializer,
} from '../models/idempotencyAbortOut';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export interface IdempotencyAbortOptions {
        idempotencyKey?: string;
        }

    export class Idempotency {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Abandon an idempotent request (remove lock without saving response) */
        public abort(
            idempotencyAbortIn: IdempotencyAbortIn,
            options?: IdempotencyAbortOptions,
            ): Promise<IdempotencyAbortOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/idempotency/abort");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
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

