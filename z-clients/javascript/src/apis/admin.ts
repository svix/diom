// this file is @generated

import { AdminAuthPolicy } from './adminAuthPolicy';
import { AdminAuthRole } from './adminAuthRole';
import { AdminAuthToken } from './adminAuthToken';
import { AdminCluster } from './adminCluster';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Admin {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get auth_policy() {
        return new AdminAuthPolicy(this.requestCtx);
    }

    public get auth_role() {
        return new AdminAuthRole(this.requestCtx);
    }

    public get auth_token() {
        return new AdminAuthToken(this.requestCtx);
    }

    public get cluster() {
        return new AdminCluster(this.requestCtx);
    }

    
}

