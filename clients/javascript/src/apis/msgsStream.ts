// this file is @generated

import {
    type StreamReceiveIn,
    StreamReceiveInSerializer,
} from '../models/streamReceiveIn';
import {
    type StreamReceiveOut,
    StreamReceiveOutSerializer,
} from '../models/streamReceiveOut';
import { HttpMethod, CoyoteRequest, CoyoteRequestContext } from "../request";

export class MsgsStream {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
        public receive(
            streamReceiveIn: StreamReceiveIn,
            ): Promise<StreamReceiveOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/stream/receive");

            request.setBody(
                    StreamReceiveInSerializer._toJsonObject(
                        streamReceiveIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    StreamReceiveOutSerializer._fromJsonObject,
                );
            }

        

    }

