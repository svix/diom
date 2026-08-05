// this file is @generated

import {
    type MsgFifoAckIn,
    MsgFifoAckInSerializer,
} from '../models/msgFifoAckIn';
import {
    type MsgFifoAckOut,
    MsgFifoAckOutSerializer,
} from '../models/msgFifoAckOut';
import {
    type MsgFifoConfigureIn,
    MsgFifoConfigureInSerializer,
} from '../models/msgFifoConfigureIn';
import {
    type MsgFifoConfigureOut,
    MsgFifoConfigureOutSerializer,
} from '../models/msgFifoConfigureOut';
import {
    type MsgFifoExtendLeaseIn,
    MsgFifoExtendLeaseInSerializer,
} from '../models/msgFifoExtendLeaseIn';
import {
    type MsgFifoExtendLeaseOut,
    MsgFifoExtendLeaseOutSerializer,
} from '../models/msgFifoExtendLeaseOut';
import {
    type MsgFifoNackIn,
    MsgFifoNackInSerializer,
} from '../models/msgFifoNackIn';
import {
    type MsgFifoNackOut,
    MsgFifoNackOutSerializer,
} from '../models/msgFifoNackOut';
import {
    type MsgFifoReceiveIn,
    MsgFifoReceiveInSerializer,
} from '../models/msgFifoReceiveIn';
import {
    type MsgFifoReceiveOut,
    MsgFifoReceiveOutSerializer,
} from '../models/msgFifoReceiveOut';
import {
    type MsgFifoRedriveDlqIn,
    MsgFifoRedriveDlqInSerializer,
} from '../models/msgFifoRedriveDlqIn';
import {
    type MsgFifoRedriveDlqOut,
    MsgFifoRedriveDlqOutSerializer,
} from '../models/msgFifoRedriveDlqOut';
import { HttpMethod, DiomRequest, type DiomRequestContext } from "../request";

export class MsgsFifo {
    public constructor(private readonly requestCtx: DiomRequestContext) {}

    /**
* Receives messages from a topic with strict per-key ordering.
* 
* Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
* message for a key, no other consumer receives that key's messages until it is acked (or its
* lease expires). A single call may return several messages of the same key, in order. Keyless
* messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
* split a key across partitions, breaking its order at that boundary.
*/
    public receive(
        topic: string,
        consumer_group: string,
        msgFifoReceiveIn: MsgFifoReceiveIn,
    ): Promise<MsgFifoReceiveOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.fifo.receive");

        request.setBody(
            MsgFifoReceiveInSerializer._toJsonObject({
                ...msgFifoReceiveIn,
                topic: topic,
                consumerGroup: consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgFifoReceiveOutSerializer._fromJsonObject,
        );
    }/** Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message. */
    public ack(
        topic: string,
        consumer_group: string,
        msgFifoAckIn: MsgFifoAckIn,
    ): Promise<MsgFifoAckOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.fifo.ack");

        request.setBody(
            MsgFifoAckInSerializer._toJsonObject({
                ...msgFifoAckIn,
                topic: topic,
                consumerGroup: consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgFifoAckOutSerializer._fromJsonObject,
        );
    }/** Extends the lease on in-flight fifo messages. */
    public extendLease(
        topic: string,
        consumer_group: string,
        msgFifoExtendLeaseIn: MsgFifoExtendLeaseIn,
    ): Promise<MsgFifoExtendLeaseOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.fifo.extend-lease");

        request.setBody(
            MsgFifoExtendLeaseInSerializer._toJsonObject({
                ...msgFifoExtendLeaseIn,
                topic: topic,
                consumerGroup: consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgFifoExtendLeaseOutSerializer._fromJsonObject,
        );
    }/** Configures retry and DLQ behavior for a fifo consumer group on a topic. */
    public configure(
        topic: string,
        consumer_group: string,
        msgFifoConfigureIn: MsgFifoConfigureIn,
    ): Promise<MsgFifoConfigureOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.fifo.configure");

        request.setBody(
            MsgFifoConfigureInSerializer._toJsonObject({
                ...msgFifoConfigureIn,
                topic: topic,
                consumerGroup: consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgFifoConfigureOutSerializer._fromJsonObject,
        );
    }/** Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ. */
    public nack(
        topic: string,
        consumer_group: string,
        msgFifoNackIn: MsgFifoNackIn,
    ): Promise<MsgFifoNackOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.fifo.nack");

        request.setBody(
            MsgFifoNackInSerializer._toJsonObject({
                ...msgFifoNackIn,
                topic: topic,
                consumerGroup: consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgFifoNackOutSerializer._fromJsonObject,
        );
    }/** Moves all dead-letter queue messages for a fifo consumer group back for reprocessing. */
    public redriveDlq(
        topic: string,
        consumer_group: string,
        msgFifoRedriveDlqIn: MsgFifoRedriveDlqIn,
    ): Promise<MsgFifoRedriveDlqOut> {
        const request = new DiomRequest(HttpMethod.POST, "/api/v1.msgs.fifo.redrive-dlq");

        request.setBody(
            MsgFifoRedriveDlqInSerializer._toJsonObject({
                ...msgFifoRedriveDlqIn,
                topic: topic,
                consumerGroup: consumer_group,
            })
        );
        
        return request.send(
            this.requestCtx,
            MsgFifoRedriveDlqOutSerializer._fromJsonObject,
        );
    }
}

