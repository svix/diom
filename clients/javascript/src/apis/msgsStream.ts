// this file is @generated

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
import { HttpMethod, CoyoteRequest, type CoyoteRequestContext } from "../request";

export class MsgsStream {
    public constructor(private readonly requestCtx: CoyoteRequestContext) {}

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
        public receive(
            msgStreamReceiveIn: MsgStreamReceiveIn,
            ): Promise<MsgStreamReceiveOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/stream/receive");

            request.setBody(
                    MsgStreamReceiveInSerializer._toJsonObject(
                        msgStreamReceiveIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    MsgStreamReceiveOutSerializer._fromJsonObject,
                );
            }

        

    /**
* Commits an offset for a consumer group on a specific partition.
* 
* The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
* successfully processed offset; future receives will start after it.
*/
        public commit(
            msgStreamCommitIn: MsgStreamCommitIn,
            ): Promise<MsgStreamCommitOut> {
            const request = new CoyoteRequest(HttpMethod.POST, "/api/v1/msgs/stream/commit");

            request.setBody(
                    MsgStreamCommitInSerializer._toJsonObject(
                        msgStreamCommitIn,
                    )
                );
            
                return request.send(
                    this.requestCtx,
                    MsgStreamCommitOutSerializer._fromJsonObject,
                );
            }

        

    }

