// this file is @generated

import {
    type MsgNamespaceConfigureIn,
    MsgNamespaceConfigureInSerializer,
} from '../models/msgNamespaceConfigureIn';
import {
    type MsgNamespaceConfigureOut,
    MsgNamespaceConfigureOutSerializer,
} from '../models/msgNamespaceConfigureOut';
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

    /** Configures a msgs namespace with the given name. */
    public configure(
        name: string,
        msgNamespaceConfigureIn: MsgNamespaceConfigureIn,
    ): Promise<MsgNamespaceConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.namespace.configure");

        request.setBody(
            MsgNamespaceConfigureInSerializer._toJsonObject({
                ...msgNamespaceConfigureIn,
                name: name,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgNamespaceConfigureOutSerializer._fromJsonObject,
        );
    }/** Gets a msgs namespace by name. */
    public get(
        name: string,
        msgNamespaceGetIn: MsgNamespaceGetIn,
    ): Promise<MsgNamespaceGetOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.namespace.get");

        request.setBody(
            MsgNamespaceGetInSerializer._toJsonObject({
                ...msgNamespaceGetIn,
                name: name,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgNamespaceGetOutSerializer._fromJsonObject,
        );
    }
}

