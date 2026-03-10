// this file is @generated

import {
    type MsgNamespaceCreateIn,
    MsgNamespaceCreateInSerializer,
} from '../models/msgNamespaceCreateIn';
import {
    type MsgNamespaceCreateOut,
    MsgNamespaceCreateOutSerializer,
} from '../models/msgNamespaceCreateOut';
import {
    type MsgNamespaceGetIn,
    MsgNamespaceGetInSerializer,
} from '../models/msgNamespaceGetIn';
import {
    type MsgNamespaceGetOut,
    MsgNamespaceGetOutSerializer,
} from '../models/msgNamespaceGetOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsNamespace {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Creates or updates a msgs namespace with the given name. */
    public create(
        msgNamespaceCreateIn: MsgNamespaceCreateIn,
        ): Promise<MsgNamespaceCreateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/namespace/create");

        request.setBody(
            MsgNamespaceCreateInSerializer._toJsonObject(
                msgNamespaceCreateIn,
            )
        );
        return request.send(
            this.requestCtx,
            MsgNamespaceCreateOutSerializer._fromJsonObject,
        );
    }/** Gets a msgs namespace by name. */
    public get(
        msgNamespaceGetIn: MsgNamespaceGetIn,
        ): Promise<MsgNamespaceGetOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/namespace/get");

        request.setBody(
            MsgNamespaceGetInSerializer._toJsonObject(
                msgNamespaceGetIn,
            )
        );
        return request.send(
            this.requestCtx,
            MsgNamespaceGetOutSerializer._fromJsonObject,
        );
    }
}

