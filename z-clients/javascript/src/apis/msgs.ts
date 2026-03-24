// this file is @generated

import {
    type MsgPublishIn,
    MsgPublishInSerializer,
} from '../models/msgPublishIn';
import {
    type MsgPublishOut,
    MsgPublishOutSerializer,
} from '../models/msgPublishOut';
import { MsgsNamespace } from './msgsNamespace';
import { MsgsQueue } from './msgsQueue';
import { MsgsStream } from './msgsStream';
import { MsgsTopic } from './msgsTopic';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class Msgs {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    public get namespace() {
        return new MsgsNamespace(this.requestCtx);
    }

    public get queue() {
        return new MsgsQueue(this.requestCtx);
    }

    public get stream() {
        return new MsgsStream(this.requestCtx);
    }

    public get topic() {
        return new MsgsTopic(this.requestCtx);
    }

    /** Publishes messages to a topic within a namespace. */
    public publish(
        topic: string,
        msgPublishIn: MsgPublishIn,
    ): Promise<MsgPublishOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/publish");

        request.setBody(
            MsgPublishInSerializer._toJsonObject({
                ...msgPublishIn,
                topic,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgPublishOutSerializer._fromJsonObject,
        );
    }
}

