// this file is @generated

import {
    type PingOut,
    PingOutSerializer,
} from '../models/pingOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Health {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Verify the server is up and running. */
    public ping(
        ): Promise<PingOut> {
        const request = new DiomRequest(HttpMethod.GET, "/api/v1/health/ping");

        
        return request.send(
            this.requestCtx,
            PingOutSerializer._fromJsonObject,
        );
    }
}

