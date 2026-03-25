// this file is @generated

import {
    type PingOut,
    PingOutSerializer,
} from '../models/pingOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class Health {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Verify the server is up and running. */
    public ping(
    ): Promise<PingOut> {
        const request = new CoyoteRequest(HttpMethod.GET, "/api/v1.health.ping");

        
        return request.send(
            this.requestCtx,
            PingOutSerializer._fromJsonObject,
        );
    }/** Intentionally return an error */
    public error(
    ): Promise<void> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.health.error");

        
        return request.sendNoResponseBody(this.requestCtx);
    }
}

