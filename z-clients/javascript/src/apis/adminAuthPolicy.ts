// this file is @generated

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
    type AdminAccessPolicyUpsertIn,
    AdminAccessPolicyUpsertInSerializer,
} from '../models/adminAccessPolicyUpsertIn';
import {
    type AdminAccessPolicyUpsertOut,
    AdminAccessPolicyUpsertOutSerializer,
} from '../models/adminAccessPolicyUpsertOut';
import {
    type ListResponseAdminAccessPolicyOut,
    ListResponseAdminAccessPolicyOutSerializer,
} from '../models/listResponseAdminAccessPolicyOut';
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class AdminAuthPolicy {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /** Create or update an access policy */
    public upsert(
        adminAccessPolicyUpsertIn: AdminAccessPolicyUpsertIn,
    ): Promise<AdminAccessPolicyUpsertOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.upsert");

        request.setBody(
            AdminAccessPolicyUpsertInSerializer._toJsonObject(adminAccessPolicyUpsertIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAccessPolicyUpsertOutSerializer._fromJsonObject,
        );
    }/** Delete an access policy */
    public delete(
        adminAccessPolicyDeleteIn: AdminAccessPolicyDeleteIn,
    ): Promise<AdminAccessPolicyDeleteOut> {
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.delete");

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
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.get");

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
        const request = new CoyoteRequest(HttpMethod.POST, "/api/v1.admin.auth-policy.list");

        request.setBody(
            AdminAccessPolicyListInSerializer._toJsonObject(adminAccessPolicyListIn)
        );
        
        return request.send(
            this.requestCtx,
            ListResponseAdminAccessPolicyOutSerializer._fromJsonObject,
        );
    }
}

