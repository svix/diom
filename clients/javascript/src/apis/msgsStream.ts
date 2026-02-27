// this file is @generated

import {
    type StreamCommitIn,
    StreamCommitInSerializer,
} from '../models/streamCommitIn';
import {
    type StreamCommitOut,
    StreamCommitOutSerializer,
} from '../models/streamCommitOut';
import {
    type StreamReceiveIn,
    StreamReceiveInSerializer,
} from '../models/streamReceiveIn';
import {
    type StreamReceiveOut,
    StreamReceiveOutSerializer,
} from '../models/streamReceiveOut';
import { HttpMethod, DiomRequest, DiomRequestContext } from "../request";

export class MsgsStream {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
        public receive(
            streamReceiveIn: StreamReceiveIn,
            ): Promise<StreamReceiveOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/stream/receive");

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

        

    /**
* Commits an offset for a consumer group on a specific partition.
* 
* The topic must be a partition-level topic (e.g. `my-topic~3`). The offset is the last
* successfully processed offset; future receives will start after it.
*/
        public commit(
            streamCommitIn: StreamCommitIn,
            ): Promise<StreamCommitOut> {
            const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/stream/commit");

            request.setBody(
                    StreamCommitInSerializer._toJsonObject(
                        streamCommitIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    StreamCommitOutSerializer._fromJsonObject,
                );
            }

        

    }

