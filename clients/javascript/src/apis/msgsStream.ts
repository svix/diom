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
import {
    type MsgStreamSeekIn,
    MsgStreamSeekInSerializer,
} from '../models/msgStreamSeekIn';
import {
    type MsgStreamSeekOut,
    MsgStreamSeekOutSerializer,
} from '../models/msgStreamSeekOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsStream {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
    public receive(
        topic: string,
        consumer_group: string,
        msgStreamReceiveIn: MsgStreamReceiveIn,
    ): Promise<MsgStreamReceiveOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/stream/receive");

        request.setBody(
            MsgStreamReceiveInSerializer._toJsonObject({
                ...msgStreamReceiveIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgStreamReceiveOutSerializer._fromJsonObject,
        );
    }/**
* Commits an offset for a consumer group on a specific partition.
* 
* The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
* successfully processed offset; future receives will start after it.
*/
    public commit(
        topic: string,
        consumer_group: string,
        msgStreamCommitIn: MsgStreamCommitIn,
    ): Promise<MsgStreamCommitOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/stream/commit");

        request.setBody(
            MsgStreamCommitInSerializer._toJsonObject({
                ...msgStreamCommitIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgStreamCommitOutSerializer._fromJsonObject,
        );
    }/**
* Repositions a consumer group's read cursor on a topic.
* 
* Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
* partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
* `"latest"` and may be used with or without a partition suffix.
*/
    public seek(
        topic: string,
        consumer_group: string,
        msgStreamSeekIn: MsgStreamSeekIn,
    ): Promise<MsgStreamSeekOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/stream/seek");

        request.setBody(
            MsgStreamSeekInSerializer._toJsonObject({
                ...msgStreamSeekIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgStreamSeekOutSerializer._fromJsonObject,
        );
    }
}

