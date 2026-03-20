// this file is @generated

import {
    type AuthTokenCreateNamespaceIn,
    AuthTokenCreateNamespaceInSerializer,
} from '../models/authTokenCreateNamespaceIn';
import {
    type AuthTokenCreateNamespaceOut,
    AuthTokenCreateNamespaceOutSerializer,
} from '../models/authTokenCreateNamespaceOut';
import {
    type AuthTokenGetNamespaceIn,
    AuthTokenGetNamespaceInSerializer,
} from '../models/authTokenGetNamespaceIn';
import {
    type AuthTokenGetNamespaceOut,
    AuthTokenGetNamespaceOutSerializer,
} from '../models/authTokenGetNamespaceOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class AuthTokenNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create Auth Token namespace */
    public create(
        authTokenCreateNamespaceIn: AuthTokenCreateNamespaceIn,
    ): Promise<AuthTokenCreateNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/namespace/create");

        request.setBody(
            AuthTokenCreateNamespaceInSerializer._toJsonObject(authTokenCreateNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenCreateNamespaceOutSerializer._fromJsonObject,
        );
    }/** Get Auth Token namespace */
    public get(
        authTokenGetNamespaceIn: AuthTokenGetNamespaceIn,
    ): Promise<AuthTokenGetNamespaceOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/auth-token/namespace/get");

        request.setBody(
            AuthTokenGetNamespaceInSerializer._toJsonObject(authTokenGetNamespaceIn)
        );
        
        return request.send(
            this.requestCtx,
            AuthTokenGetNamespaceOutSerializer._fromJsonObject,
        );
    }
}

