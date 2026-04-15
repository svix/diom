// this file is @generated

import {
    type AdminRoleConfigureIn,
    AdminRoleConfigureInSerializer,
} from '../models/adminRoleConfigureIn';
import {
    type AdminRoleConfigureOut,
    AdminRoleConfigureOutSerializer,
} from '../models/adminRoleConfigureOut';
import {
    type AdminRoleDeleteIn,
    AdminRoleDeleteInSerializer,
} from '../models/adminRoleDeleteIn';
import {
    type AdminRoleDeleteOut,
    AdminRoleDeleteOutSerializer,
} from '../models/adminRoleDeleteOut';
import {
    type AdminRoleGetIn,
    AdminRoleGetInSerializer,
} from '../models/adminRoleGetIn';
import {
    type AdminRoleListIn,
    AdminRoleListInSerializer,
} from '../models/adminRoleListIn';
import {
    type AdminRoleOut,
    AdminRoleOutSerializer,
} from '../models/adminRoleOut';
import {
    type ListResponseAdminRoleOut,
    ListResponseAdminRoleOutSerializer,
} from '../models/listResponseAdminRoleOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AdminAuthRole {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create or update a role */
    public configure(
        adminRoleConfigureIn: AdminRoleConfigureIn,
    ): Promise<AdminRoleConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-role.configure");

        request.setBody(
            AdminRoleConfigureInSerializer._toJsonObject(adminRoleConfigureIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminRoleConfigureOutSerializer._fromJsonObject,
        );
    }/** Delete a role */
    public delete(
        adminRoleDeleteIn: AdminRoleDeleteIn,
    ): Promise<AdminRoleDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-role.delete");

        request.setBody(
            AdminRoleDeleteInSerializer._toJsonObject(adminRoleDeleteIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminRoleDeleteOutSerializer._fromJsonObject,
        );
    }/** Get a role by ID */
    public get(
        adminRoleGetIn: AdminRoleGetIn,
    ): Promise<AdminRoleOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-role.get");

        request.setBody(
            AdminRoleGetInSerializer._toJsonObject(adminRoleGetIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminRoleOutSerializer._fromJsonObject,
        );
    }/** List all roles */
    public list(
        adminRoleListIn: AdminRoleListIn,
    ): Promise<ListResponseAdminRoleOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-role.list");

        request.setBody(
            AdminRoleListInSerializer._toJsonObject(adminRoleListIn)
        );
        
        return request.send(
            this.requestCtx,
            ListResponseAdminRoleOutSerializer._fromJsonObject,
        );
    }
}

