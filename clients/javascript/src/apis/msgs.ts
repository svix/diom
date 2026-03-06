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
import {
    type MsgPublishIn,
    MsgPublishInSerializer,
} from '../models/msgPublishIn';
import {
    type MsgPublishOut,
    MsgPublishOutSerializer,
} from '../models/msgPublishOut';
import {
    type MsgQueueAckIn,
    MsgQueueAckInSerializer,
} from '../models/msgQueueAckIn';
import {
    type MsgQueueAckOut,
    MsgQueueAckOutSerializer,
} from '../models/msgQueueAckOut';
import {
    type MsgQueueReceiveIn,
    MsgQueueReceiveInSerializer,
} from '../models/msgQueueReceiveIn';
import {
    type MsgQueueReceiveOut,
    MsgQueueReceiveOutSerializer,
} from '../models/msgQueueReceiveOut';
import {
    type MsgStreamCommitIn,
    MsgStreamCommitInSerializer,
} from '../models/msgStreamCommitIn';
import {
    type MsgStreamCommitOut,
    MsgStreamCommitOutSerializer,
} from '../models/msgStreamCommitOut';
import {
    type MsgStreamReceiveIn,
    MsgStreamReceiveInSerializer,
} from '../models/msgStreamReceiveIn';
import {
    type MsgStreamReceiveOut,
    MsgStreamReceiveOutSerializer,
} from '../models/msgStreamReceiveOut';
import {
    type MsgTopicConfigureIn,
    MsgTopicConfigureInSerializer,
} from '../models/msgTopicConfigureIn';
import {
    type MsgTopicConfigureOut,
    MsgTopicConfigureOutSerializer,
} from '../models/msgTopicConfigureOut';
import { MsgsNamespace } from './msgsNamespace';
import { MsgsQueue } from './msgsQueue';
import { MsgsStream } from './msgsStream';
import { MsgsTopic } from './msgsTopic';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class Msgs {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

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
            msgPublishIn: MsgPublishIn,
            ): Promise<MsgPublishOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/publish");

            request.setBody(
                    MsgPublishInSerializer._toJsonObject(
                        msgPublishIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    MsgPublishOutSerializer._fromJsonObject,
                );
            }

        

    }

