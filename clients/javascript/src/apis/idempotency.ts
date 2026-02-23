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
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export interface IdempotencyAbortOptions {
        idempotencyKey?: string;
        }

    export interface IdempotencyGetGroupOptions {
        idempotencyKey?: string;
        }

    export class Idempotency {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Abandon an idempotent request (remove lock without saving response) */
        public abort(
            idempotencyAbortIn: IdempotencyAbortIn,
            options?: IdempotencyAbortOptions,
            ): Promise<IdempotencyAbortOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/idempotency/abort");

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

        

    /** Get idempotency group */
        public getGroup(
            idempotencyGetGroupIn: IdempotencyGetGroupIn,
            options?: IdempotencyGetGroupOptions,
            ): Promise<IdempotencyGetGroupOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/idempotency/get-group");

            request.setHeaderParam("idempotency-key", options?.idempotencyKey);
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

