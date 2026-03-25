// this file is @generated

import { AdminCluster } from './adminCluster';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get cluster() {
        return new AdminCluster(this.requestCtx);
    }

    
}

