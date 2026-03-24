// this file is @generated

import {
    type MsgQueueAckIn,
    MsgQueueAckInSerializer,
} from '../models/msgQueueAckIn';
import {
    type MsgQueueAckOut,
    MsgQueueAckOutSerializer,
} from '../models/msgQueueAckOut';
import {
    type MsgQueueConfigureIn,
    MsgQueueConfigureInSerializer,
} from '../models/msgQueueConfigureIn';
import {
    type MsgQueueConfigureOut,
    MsgQueueConfigureOutSerializer,
} from '../models/msgQueueConfigureOut';
import {
    type MsgQueueNackIn,
    MsgQueueNackInSerializer,
} from '../models/msgQueueNackIn';
import {
    type MsgQueueNackOut,
    MsgQueueNackOutSerializer,
} from '../models/msgQueueNackOut';
import {
    type MsgQueueReceiveIn,
    MsgQueueReceiveInSerializer,
} from '../models/msgQueueReceiveIn';
import {
    type MsgQueueReceiveOut,
    MsgQueueReceiveOutSerializer,
} from '../models/msgQueueReceiveOut';
import {
    type MsgQueueRedriveDlqIn,
    MsgQueueRedriveDlqInSerializer,
} from '../models/msgQueueRedriveDlqIn';
import {
    type MsgQueueRedriveDlqOut,
    MsgQueueRedriveDlqOutSerializer,
} from '../models/msgQueueRedriveDlqOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsQueue {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Receives messages from a topic as competing consumers.
* 
* Messages are individually leased for the specified duration. Multiple consumers can receive
* different messages from the same topic concurrently. Leased messages are skipped until they
* are acked or their lease expires.
*/
    public receive(
        topic: string,
        consumer_group: string,
        msgQueueReceiveIn: MsgQueueReceiveIn,
    ): Promise<MsgQueueReceiveOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/receive");

        request.setBody(
            MsgQueueReceiveInSerializer._toJsonObject({
                ...msgQueueReceiveIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgQueueReceiveOutSerializer._fromJsonObject,
        );
    }/**
* Acknowledges messages by their opaque msg_ids.
* 
* Acked messages are permanently removed from the queue and will never be re-delivered.
*/
    public ack(
        topic: string,
        consumer_group: string,
        msgQueueAckIn: MsgQueueAckIn,
    ): Promise<MsgQueueAckOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/ack");

        request.setBody(
            MsgQueueAckInSerializer._toJsonObject({
                ...msgQueueAckIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgQueueAckOutSerializer._fromJsonObject,
        );
    }/**
* Configures retry and DLQ behavior for a consumer group on a topic.
* 
* `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
* the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
*/
    public configure(
        topic: string,
        consumer_group: string,
        msgQueueConfigureIn: MsgQueueConfigureIn,
    ): Promise<MsgQueueConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/configure");

        request.setBody(
            MsgQueueConfigureInSerializer._toJsonObject({
                ...msgQueueConfigureIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgQueueConfigureOutSerializer._fromJsonObject,
        );
    }/**
* Rejects messages, sending them to the dead-letter queue.
* 
* Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
* move them back to the queue for reprocessing.
*/
    public nack(
        topic: string,
        consumer_group: string,
        msgQueueNackIn: MsgQueueNackIn,
    ): Promise<MsgQueueNackOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/nack");

        request.setBody(
            MsgQueueNackInSerializer._toJsonObject({
                ...msgQueueNackIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgQueueNackOutSerializer._fromJsonObject,
        );
    }/** Moves all dead-letter queue messages back to the main queue for reprocessing. */
    public redriveDlq(
        topic: string,
        consumer_group: string,
        msgQueueRedriveDlqIn: MsgQueueRedriveDlqIn,
    ): Promise<MsgQueueRedriveDlqOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1/msgs/queue/redrive-dlq");

        request.setBody(
            MsgQueueRedriveDlqInSerializer._toJsonObject({
                ...msgQueueRedriveDlqIn,
                topic,
                consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgQueueRedriveDlqOutSerializer._fromJsonObject,
        );
    }
}

