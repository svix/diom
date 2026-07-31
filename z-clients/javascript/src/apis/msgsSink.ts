// this file is @generated

import {
    type ListResponseSinkOut,
    ListResponseSinkOutSerializer,
} from '../models/listResponseSinkOut';
import {
    type SinkConfigureIn,
    SinkConfigureInSerializer,
} from '../models/sinkConfigureIn';
import {
    type SinkConfigureOut,
    SinkConfigureOutSerializer,
} from '../models/sinkConfigureOut';
import {
    type SinkDeleteIn,
    SinkDeleteInSerializer,
} from '../models/sinkDeleteIn';
import {
    type SinkDeleteOut,
    SinkDeleteOutSerializer,
} from '../models/sinkDeleteOut';
import {
    type SinkListIn,
    SinkListInSerializer,
} from '../models/sinkListIn';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsSink {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /** Create or update a sink for a topic. Overwrites any existing sink with the same id. */
    public configure(
        sinkConfigureIn: SinkConfigureIn,
    ): Promise<SinkConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.sink.configure");

        request.setBody(
            SinkConfigureInSerializer._toJsonObject(sinkConfigureIn)
        );
        
        return request.send(
            this.requestCtx,
            SinkConfigureOutSerializer._fromJsonObject,
        );
    }/** Delete a sink. */
    public delete(
        sinkDeleteIn: SinkDeleteIn,
    ): Promise<SinkDeleteOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.sink.delete");

        request.setBody(
            SinkDeleteInSerializer._toJsonObject(sinkDeleteIn)
        );
        
        return request.send(
            this.requestCtx,
            SinkDeleteOutSerializer._fromJsonObject,
        );
    }/** List sink configurations for a topic. */
    public list(
        topic: string,
        sinkListIn: SinkListIn,
    ): Promise<ListResponseSinkOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.sink.list");

        request.setBody(
            SinkListInSerializer._toJsonObject({
                ...sinkListIn,
                topic: topic,
            })
        );
        
        return request.send(
            this.requestCtx,
            ListResponseSinkOutSerializer._fromJsonObject,
        );
    }
}

