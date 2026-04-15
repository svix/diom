// this file is @generated

import {
    type AdminAccessPolicyConfigureIn,
    AdminAccessPolicyConfigureInSerializer,
} from '../models/adminAccessPolicyConfigureIn';
import {
    type AdminAccessPolicyConfigureOut,
    AdminAccessPolicyConfigureOutSerializer,
} from '../models/adminAccessPolicyConfigureOut';
import {
    type AdminAccessPolicyDeleteIn,
    AdminAccessPolicyDeleteInSerializer,
} from '../models/adminAccessPolicyDeleteIn';
import {
    type AdminAccessPolicyDeleteOut,
    AdminAccessPolicyDeleteOutSerializer,
} from '../models/adminAccessPolicyDeleteOut';
import {
    type AdminAccessPolicyGetIn,
    AdminAccessPolicyGetInSerializer,
} from '../models/adminAccessPolicyGetIn';
import {
    type AdminAccessPolicyListIn,
    AdminAccessPolicyListInSerializer,
} from '../models/adminAccessPolicyListIn';
import {
    type AdminAccessPolicyOut,
    AdminAccessPolicyOutSerializer,
} from '../models/adminAccessPolicyOut';
import {
    type ListResponseAdminAccessPolicyOut,
    ListResponseAdminAccessPolicyOutSerializer,
} from '../models/listResponseAdminAccessPolicyOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AdminAuthPolicy {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create or update an access policy */
    public configure(
        adminAccessPolicyConfigureIn: AdminAccessPolicyConfigureIn,
    ): Promise<AdminAccessPolicyConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.configure");

        request.setBody(
            AdminAccessPolicyConfigureInSerializer._toJsonObject(adminAccessPolicyConfigureIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAccessPolicyConfigureOutSerializer._fromJsonObject,
        );
    }/** Delete an access policy */
    public delete(
        adminAccessPolicyDeleteIn: AdminAccessPolicyDeleteIn,
    ): Promise<AdminAccessPolicyDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.delete");

        request.setBody(
            AdminAccessPolicyDeleteInSerializer._toJsonObject(adminAccessPolicyDeleteIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAccessPolicyDeleteOutSerializer._fromJsonObject,
        );
    }/** Get an access policy by ID */
    public get(
        adminAccessPolicyGetIn: AdminAccessPolicyGetIn,
    ): Promise<AdminAccessPolicyOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.get");

        request.setBody(
            AdminAccessPolicyGetInSerializer._toJsonObject(adminAccessPolicyGetIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAccessPolicyOutSerializer._fromJsonObject,
        );
    }/** List all access policies */
    public list(
        adminAccessPolicyListIn: AdminAccessPolicyListIn,
    ): Promise<ListResponseAdminAccessPolicyOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.list");

        request.setBody(
            AdminAccessPolicyListInSerializer._toJsonObject(adminAccessPolicyListIn)
        );
        
        return request.send(
            this.requestCtx,
            ListResponseAdminAccessPolicyOutSerializer._fromJsonObject,
        );
    }
}

