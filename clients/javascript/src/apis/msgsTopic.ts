// this file is @generated

import {
    type MsgTopicConfigureIn,
    MsgTopicConfigureInSerializer,
} from '../models/msgTopicConfigureIn';
import {
    type MsgTopicConfigureOut,
    MsgTopicConfigureOutSerializer,
} from '../models/msgTopicConfigureOut';
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class MsgsTopic {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Configures the number of partitions for a topic.
* 
* Partition count can only be increased, never decreased. The default for a new topic is 1.
*/
        public configure(
            msgTopicConfigureIn: MsgTopicConfigureIn,
            ): Promise<MsgTopicConfigureOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/topic/configure");

            request.setBody(
                    MsgTopicConfigureInSerializer._toJsonObject(
                        msgTopicConfigureIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    MsgTopicConfigureOutSerializer._fromJsonObject,
                );
            }

        

    }

