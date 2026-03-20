// this file is @generated

import { AdminAuthToken } from './adminAuthToken';
import { AdminCluster } from './adminCluster';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get auth_token() {
        return new AdminAuthToken(this.requestCtx);
    }

    public get cluster() {
        return new AdminCluster(this.requestCtx);
    }

    
}

