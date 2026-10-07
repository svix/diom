// this file is @generated

import {
    type ListResponseMsgTopicListOut,
    ListResponseMsgTopicListOutSerializer,
} from '../models/listResponseMsgTopicListOut';
import {
    type MsgTopicConfigureIn,
    MsgTopicConfigureInSerializer,
} from '../models/msgTopicConfigureIn';
import {
    type MsgTopicConfigureOut,
    MsgTopicConfigureOutSerializer,
} from '../models/msgTopicConfigureOut';
import {
    type MsgTopicListIn,
    MsgTopicListInSerializer,
} from '../models/msgTopicListIn';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsTopic {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Configures the number of partitions for a topic.
* 
* Partition count can only be increased, never decreased. The default for a new topic is 1.
*/
    public configure(
        topic: string,
        msgTopicConfigureIn: MsgTopicConfigureIn,
    ): Promise<MsgTopicConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.topic.configure");

        request.setBody(
            MsgTopicConfigureInSerializer._toJsonObject({
                ...msgTopicConfigureIn,
                topic: topic,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgTopicConfigureOutSerializer._fromJsonObject,
        );
    }/** List available topics in the given namespace */
    public list(
        msgTopicListIn: MsgTopicListIn,
    ): Promise<ListResponseMsgTopicListOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.topic.list");

        request.setBody(
            MsgTopicListInSerializer._toJsonObject(msgTopicListIn)
        );
        
        return request.send(
            this.requestCtx,
            ListResponseMsgTopicListOutSerializer._fromJsonObject,
        );
    }
}

