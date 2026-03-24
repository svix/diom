// this file is @generated

import { AdminCluster } from './adminCluster';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    public get cluster() {
        return new AdminCluster(this.requestCtx);
    }

    
}

