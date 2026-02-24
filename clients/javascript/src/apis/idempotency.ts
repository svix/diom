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
    type IdempotencyGetGroupIn,
    IdempotencyGetGroupInSerializer,
} from '../models/idempotencyGetGroupIn';
import {
    type IdempotencyGetGroupOut,
    IdempotencyGetGroupOutSerializer,
} from '../models/idempotencyGetGroupOut';
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

        

    /** Get idempotency group */
        public getGroup(
            idempotencyGetGroupIn: IdempotencyGetGroupIn,
            ): Promise<IdempotencyGetGroupOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/idempotency/get-group");

            request.setBody(
                    IdempotencyGetGroupInSerializer._toJsonObject(
                        idempotencyGetGroupIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    IdempotencyGetGroupOutSerializer._fromJsonObject,
                );
            }

        

    }

