// this file is @generated

import {
    type AdminAuthTokenCreateIn,
    AdminAuthTokenCreateInSerializer,
} from '../models/adminAuthTokenCreateIn';
import {
    type AdminAuthTokenCreateOut,
    AdminAuthTokenCreateOutSerializer,
} from '../models/adminAuthTokenCreateOut';
import {
    type AdminAuthTokenDeleteIn,
    AdminAuthTokenDeleteInSerializer,
} from '../models/adminAuthTokenDeleteIn';
import {
    type AdminAuthTokenDeleteOut,
    AdminAuthTokenDeleteOutSerializer,
} from '../models/adminAuthTokenDeleteOut';
import {
    type AdminAuthTokenExpireIn,
    AdminAuthTokenExpireInSerializer,
} from '../models/adminAuthTokenExpireIn';
import {
    type AdminAuthTokenExpireOut,
    AdminAuthTokenExpireOutSerializer,
} from '../models/adminAuthTokenExpireOut';
import {
    type AdminAuthTokenListIn,
    AdminAuthTokenListInSerializer,
} from '../models/adminAuthTokenListIn';
import {
    type AdminAuthTokenRotateIn,
    AdminAuthTokenRotateInSerializer,
} from '../models/adminAuthTokenRotateIn';
import {
    type AdminAuthTokenRotateOut,
    AdminAuthTokenRotateOutSerializer,
} from '../models/adminAuthTokenRotateOut';
import {
    type AdminAuthTokenUpdateIn,
    AdminAuthTokenUpdateInSerializer,
} from '../models/adminAuthTokenUpdateIn';
import {
    type AdminAuthTokenUpdateOut,
    AdminAuthTokenUpdateOutSerializer,
} from '../models/adminAuthTokenUpdateOut';
import {
    type AdminAuthTokenWhoamiIn,
    AdminAuthTokenWhoamiInSerializer,
} from '../models/adminAuthTokenWhoamiIn';
import {
    type AdminAuthTokenWhoamiOut,
    AdminAuthTokenWhoamiOutSerializer,
} from '../models/adminAuthTokenWhoamiOut';
import {
    type ListResponseAdminAuthTokenOut,
    ListResponseAdminAuthTokenOutSerializer,
} from '../models/listResponseAdminAuthTokenOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AdminAuthToken {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create an auth token */
    public create(
        adminAuthTokenCreateIn: AdminAuthTokenCreateIn,
    ): Promise<AdminAuthTokenCreateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.create");

        request.setBody(
            AdminAuthTokenCreateInSerializer._toJsonObject(adminAuthTokenCreateIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAuthTokenCreateOutSerializer._fromJsonObject,
        );
    }/** Expire an auth token */
    public expire(
        adminAuthTokenExpireIn: AdminAuthTokenExpireIn,
    ): Promise<AdminAuthTokenExpireOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.expire");

        request.setBody(
            AdminAuthTokenExpireInSerializer._toJsonObject(adminAuthTokenExpireIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAuthTokenExpireOutSerializer._fromJsonObject,
        );
    }/** Rotate an auth token, invalidating the old one and issuing a new secret */
    public rotate(
        adminAuthTokenRotateIn: AdminAuthTokenRotateIn,
    ): Promise<AdminAuthTokenRotateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.rotate");

        request.setBody(
            AdminAuthTokenRotateInSerializer._toJsonObject(adminAuthTokenRotateIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAuthTokenRotateOutSerializer._fromJsonObject,
        );
    }/** Delete an auth token */
    public delete(
        adminAuthTokenDeleteIn: AdminAuthTokenDeleteIn,
    ): Promise<AdminAuthTokenDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.delete");

        request.setBody(
            AdminAuthTokenDeleteInSerializer._toJsonObject(adminAuthTokenDeleteIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAuthTokenDeleteOutSerializer._fromJsonObject,
        );
    }/** List auth tokens for a given owner */
    public list(
        adminAuthTokenListIn: AdminAuthTokenListIn,
    ): Promise<ListResponseAdminAuthTokenOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.list");

        request.setBody(
            AdminAuthTokenListInSerializer._toJsonObject(adminAuthTokenListIn)
        );
        
        return request.send(
            this.requestCtx,
            ListResponseAdminAuthTokenOutSerializer._fromJsonObject,
        );
    }/** Update an auth token's properties */
    public update(
        adminAuthTokenUpdateIn: AdminAuthTokenUpdateIn,
    ): Promise<AdminAuthTokenUpdateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.update");

        request.setBody(
            AdminAuthTokenUpdateInSerializer._toJsonObject(adminAuthTokenUpdateIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAuthTokenUpdateOutSerializer._fromJsonObject,
        );
    }/** Return the role of the currently authenticated token */
    public whoami(
        adminAuthTokenWhoamiIn: AdminAuthTokenWhoamiIn,
    ): Promise<AdminAuthTokenWhoamiOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.admin.auth-token.whoami");

        request.setBody(
            AdminAuthTokenWhoamiInSerializer._toJsonObject(adminAuthTokenWhoamiIn)
        );
        
        return request.send(
            this.requestCtx,
            AdminAuthTokenWhoamiOutSerializer._fromJsonObject,
        );
    }
}

