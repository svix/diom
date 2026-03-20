// this file is @generated

import {
    type AuthTokenCreateIn,
    AuthTokenCreateInSerializer,
} from '../models/authTokenCreateIn';
import {
    type AuthTokenCreateOut,
    AuthTokenCreateOutSerializer,
} from '../models/authTokenCreateOut';
import {
    type AuthTokenDeleteIn,
    AuthTokenDeleteInSerializer,
} from '../models/authTokenDeleteIn';
import {
    type AuthTokenDeleteOut,
    AuthTokenDeleteOutSerializer,
} from '../models/authTokenDeleteOut';
import {
    type AuthTokenExpireIn,
    AuthTokenExpireInSerializer,
} from '../models/authTokenExpireIn';
import {
    type AuthTokenExpireOut,
    AuthTokenExpireOutSerializer,
} from '../models/authTokenExpireOut';
import {
    type AuthTokenListIn,
    AuthTokenListInSerializer,
} from '../models/authTokenListIn';
import {
    type AuthTokenUpdateIn,
    AuthTokenUpdateInSerializer,
} from '../models/authTokenUpdateIn';
import {
    type AuthTokenUpdateOut,
    AuthTokenUpdateOutSerializer,
} from '../models/authTokenUpdateOut';
import {
    type AuthTokenVerifyIn,
    AuthTokenVerifyInSerializer,
} from '../models/authTokenVerifyIn';
import {
    type AuthTokenVerifyOut,
    AuthTokenVerifyOutSerializer,
} from '../models/authTokenVerifyOut';
import {
    type ListResponseAuthTokenOut,
    ListResponseAuthTokenOutSerializer,
} from '../models/listResponseAuthTokenOut';
import { AuthTokenNamespace } from './authTokenNamespace';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AuthToken {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new AuthTokenNamespace(this.requestCtx);
    }

    /** Create Auth Token */
    public create(
        authTokenCreateIn: AuthTokenCreateIn,
    ): Promise<AuthTokenCreateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/create");

        request.setBody(
            AuthTokenCreateInSerializer._toJsonObject(authTokenCreateIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenCreateOutSerializer._fromJsonObject,
        );
    }/** Expire Auth Token */
    public expire(
        authTokenExpireIn: AuthTokenExpireIn,
    ): Promise<AuthTokenExpireOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/expire");

        request.setBody(
            AuthTokenExpireInSerializer._toJsonObject(authTokenExpireIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenExpireOutSerializer._fromJsonObject,
        );
    }/** Delete Auth Token */
    public delete(
        authTokenDeleteIn: AuthTokenDeleteIn,
    ): Promise<AuthTokenDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/delete");

        request.setBody(
            AuthTokenDeleteInSerializer._toJsonObject(authTokenDeleteIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenDeleteOutSerializer._fromJsonObject,
        );
    }/** Verify Auth Token */
    public verify(
        authTokenVerifyIn: AuthTokenVerifyIn,
    ): Promise<AuthTokenVerifyOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/verify");

        request.setBody(
            AuthTokenVerifyInSerializer._toJsonObject(authTokenVerifyIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenVerifyOutSerializer._fromJsonObject,
        );
    }/** List Auth Tokens */
    public list(
        authTokenListIn: AuthTokenListIn,
    ): Promise<ListResponseAuthTokenOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/list");

        request.setBody(
            AuthTokenListInSerializer._toJsonObject(authTokenListIn)
        );
        
        return request.send(
            this.requestCtx,
            ListResponseAuthTokenOutSerializer._fromJsonObject,
        );
    }/** Update Auth Token */
    public update(
        authTokenUpdateIn: AuthTokenUpdateIn,
    ): Promise<AuthTokenUpdateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/update");

        request.setBody(
            AuthTokenUpdateInSerializer._toJsonObject(authTokenUpdateIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenUpdateOutSerializer._fromJsonObject,
        );
    }
}

