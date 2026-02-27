// this file is @generated

import {
    type TopicConfigureIn,
    TopicConfigureInSerializer,
} from '../models/topicConfigureIn';
import {
    type TopicConfigureOut,
    TopicConfigureOutSerializer,
} from '../models/topicConfigureOut';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class MsgsTopic {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /**
* Configures the number of partitions for a topic.
* 
* Partition count can only be increased, never decreased. The default for a new topic is 1.
*/
        public configure(
            topicConfigureIn: TopicConfigureIn,
            ): Promise<TopicConfigureOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/topic/configure");

            request.setBody(
                    TopicConfigureInSerializer._toJsonObject(
                        topicConfigureIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    TopicConfigureOutSerializer._fromJsonObject,
                );
            }

        

    }

