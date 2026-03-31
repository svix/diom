// this file is @generated

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
    type AdminRoleUpsertIn,
    AdminRoleUpsertInSerializer,
} from '../models/adminRoleUpsertIn';
import {
    type AdminRoleUpsertOut,
    AdminRoleUpsertOutSerializer,
} from '../models/adminRoleUpsertOut';
import {
    type ListResponseAdminRoleOut,
    ListResponseAdminRoleOutSerializer,
} from '../models/listResponseAdminRoleOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AdminAuthRole {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create or update a role */
    public upsert(
        adminRoleUpsertIn: AdminRoleUpsertIn,
    ): Promise<AdminRoleUpsertOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-role.upsert");

        request.setBody(
            AdminRoleUpsertInSerializer._toJsonObject(adminRoleUpsertIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminRoleUpsertOutSerializer._fromJsonObject,
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

