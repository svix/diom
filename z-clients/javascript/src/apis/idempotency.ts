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
    type IdempotencyCompleteIn,
    IdempotencyCompleteInSerializer,
} from '../models/idempotencyCompleteIn';
import {
    type IdempotencyCompleteOut,
    IdempotencyCompleteOutSerializer,
} from '../models/idempotencyCompleteOut';
import {
    type IdempotencyStartIn,
    IdempotencyStartInSerializer,
} from '../models/idempotencyStartIn';
import {
    type IdempotencyStartOut,
    IdempotencyStartOutSerializer,
} from '../models/idempotencyStartOut';
import { IdempotencyNamespace } from './idempotencyNamespace';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Idempotency {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new IdempotencyNamespace(this.requestCtx);
    }

    /** Start an idempotent request */
    public start(
        key: string,
        idempotencyStartIn: IdempotencyStartIn,
    ): Promise<IdempotencyStartOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.idempotency.start");

        request.setBody(
            IdempotencyStartInSerializer._toJsonObject({
                ...idempotencyStartIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            IdempotencyStartOutSerializer._fromJsonObject,
        );
    }/** Complete an idempotent request with a response */
    public complete(
        key: string,
        idempotencyCompleteIn: IdempotencyCompleteIn,
    ): Promise<IdempotencyCompleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.idempotency.complete");

        request.setBody(
            IdempotencyCompleteInSerializer._toJsonObject({
                ...idempotencyCompleteIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            IdempotencyCompleteOutSerializer._fromJsonObject,
        );
    }/** Abandon an idempotent request (remove lock without saving response) */
    public abort(
        key: string,
        idempotencyAbortIn: IdempotencyAbortIn,
    ): Promise<IdempotencyAbortOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.idempotency.abort");

        request.setBody(
            IdempotencyAbortInSerializer._toJsonObject({
                ...idempotencyAbortIn,
                key: key,
            })
        );
        
        return request.send(
            this.requestCtx,
            IdempotencyAbortOutSerializer._fromJsonObject,
        );
    }
}

