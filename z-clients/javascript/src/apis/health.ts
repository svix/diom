// this file is @generated

import {
    type PingOut,
    PingOutSerializer,
} from '../models/pingOut';
import {
    type ReadyOut,
    ReadyOutSerializer,
} from '../models/readyOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Health {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Verify the server is up and running.
* 
* This endpoint only checks the server itself, not the cluster mechanism, and should not be used
* as a readiness gate.
*/
    public ping(
    ): Promise<PingOut> {
        const request = new DiomRequest(HttpMethod.GET, "/api/v1.health.ping");

        
        return request.send(
            this.requestCtx,
            PingOutSerializer._fromJsonObject,
        );
    }/** Verify that this server is ready to serve customer traffic. */
    public ready(
    ): Promise<ReadyOut> {
        const request = new DiomRequest(HttpMethod.GET, "/api/v1.health.ready");

        
        return request.send(
            this.requestCtx,
            ReadyOutSerializer._fromJsonObject,
        );
    }/** Intentionally return an error */
    public error(
    ): Promise<void> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.health.error");

        
        return request.sendNoResponseBody(this.requestCtx);
    }
}

