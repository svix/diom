// this file is @generated

import {
    type ListResponseSvixPollerOut,
    ListResponseSvixPollerOutSerializer,
} from '../models/listResponseSvixPollerOut';
import {
    type SvixPollerCreateIn,
    SvixPollerCreateInSerializer,
} from '../models/svixPollerCreateIn';
import {
    type SvixPollerCreateOut,
    SvixPollerCreateOutSerializer,
} from '../models/svixPollerCreateOut';
import {
    type SvixPollerDeleteIn,
    SvixPollerDeleteInSerializer,
} from '../models/svixPollerDeleteIn';
import {
    type SvixPollerDeleteOut,
    SvixPollerDeleteOutSerializer,
} from '../models/svixPollerDeleteOut';
import {
    type SvixPollerListIn,
    SvixPollerListInSerializer,
} from '../models/svixPollerListIn';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsSvixPoller {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create a Svix poller configuration for a topic. */
    public create(
        svixPollerCreateIn: SvixPollerCreateIn,
    ): Promise<SvixPollerCreateOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.svix-poller.create");

        request.setBody(
            SvixPollerCreateInSerializer._toJsonObject(svixPollerCreateIn)
        );
        
        return request.send(
            this.requestCtx,
            SvixPollerCreateOutSerializer._fromJsonObject,
        );
    }/** Delete a Svix poller configuration. */
    public delete(
        svixPollerDeleteIn: SvixPollerDeleteIn,
    ): Promise<SvixPollerDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.svix-poller.delete");

        request.setBody(
            SvixPollerDeleteInSerializer._toJsonObject(svixPollerDeleteIn)
        );
        
        return request.send(
            this.requestCtx,
            SvixPollerDeleteOutSerializer._fromJsonObject,
        );
    }/** List Svix poller configurations for a topic. */
    public list(
        topic: string,
        svixPollerListIn: SvixPollerListIn,
    ): Promise<ListResponseSvixPollerOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.svix-poller.list");

        request.setBody(
            SvixPollerListInSerializer._toJsonObject({
                ...svixPollerListIn,
                topic: topic,
            })
        );
        
        return request.send(
            this.requestCtx,
            ListResponseSvixPollerOutSerializer._fromJsonObject,
        );
    }
}

