// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.DiomException;
import com.svix.diom.HttpClient;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import com.svix.diom.models.MsgFifoAckIn;
import com.svix.diom.models.MsgFifoAckOut;
import com.svix.diom.models.MsgFifoConfigureIn;
import com.svix.diom.models.MsgFifoConfigureOut;
import com.svix.diom.models.MsgFifoExtendLeaseIn;
import com.svix.diom.models.MsgFifoExtendLeaseOut;
import com.svix.diom.models.MsgFifoNackIn;
import com.svix.diom.models.MsgFifoNackOut;
import com.svix.diom.models.MsgFifoReceiveIn;
import com.svix.diom.models.MsgFifoReceiveOut;
import com.svix.diom.models.MsgFifoRedriveDlqIn;
import com.svix.diom.models.MsgFifoRedriveDlqOut;
import com.svix.diom.models.MsgFifoReceiveIn_;
import com.svix.diom.models.MsgFifoAckIn_;
import com.svix.diom.models.MsgFifoExtendLeaseIn_;
import com.svix.diom.models.MsgFifoConfigureIn_;
import com.svix.diom.models.MsgFifoNackIn_;
import com.svix.diom.models.MsgFifoRedriveDlqIn_;

public class MsgsFifo {
    private final HttpClient client;

    public MsgsFifo(HttpClient client) {
        this.client = client;
    }

    /**
* Receives messages from a topic with strict per-key ordering.
* 
* Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
* message for a key, no other consumer receives that key's messages until it is acked (or its
* lease expires). A single call may return several messages of the same key, in order. Keyless
* messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
* split a key across partitions, breaking its order at that boundary.
*/
    public MsgFifoReceiveOut receive(
        String topic,
        String consumerGroup,
        final MsgFifoReceiveIn msgFifoReceiveIn
    ) throws DiomException {
        MsgFifoReceiveIn_ body = new MsgFifoReceiveIn_(
            msgFifoReceiveIn.getNamespace(),
            topic,
            consumerGroup,
            msgFifoReceiveIn.getBatchSize(),
            msgFifoReceiveIn.getLeaseDuration(),
            msgFifoReceiveIn.getBatchWait()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.fifo.receive",
            null,
            body,
            MsgFifoReceiveOut.class
        );
    }

    /**
* Receives messages from a topic with strict per-key ordering.
* 
* Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
* message for a key, no other consumer receives that key's messages until it is acked (or its
* lease expires). A single call may return several messages of the same key, in order. Keyless
* messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
* split a key across partitions, breaking its order at that boundary.
*/
    public MsgFifoReceiveOut receive(
        String topic,
        String consumerGroup
    ) throws DiomException {
        return this.receive(
            topic,
            consumerGroup,
            new MsgFifoReceiveIn()
        );
    }

    /** Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message. */
    public MsgFifoAckOut ack(
        String topic,
        String consumerGroup,
        final MsgFifoAckIn msgFifoAckIn
    ) throws DiomException {
        MsgFifoAckIn_ body = new MsgFifoAckIn_(
            msgFifoAckIn.getNamespace(),
            topic,
            consumerGroup,
            msgFifoAckIn.getMsgIds()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.fifo.ack",
            null,
            body,
            MsgFifoAckOut.class
        );
    }

    /** Extends the lease on in-flight fifo messages. */
    public MsgFifoExtendLeaseOut extendLease(
        String topic,
        String consumerGroup,
        final MsgFifoExtendLeaseIn msgFifoExtendLeaseIn
    ) throws DiomException {
        MsgFifoExtendLeaseIn_ body = new MsgFifoExtendLeaseIn_(
            msgFifoExtendLeaseIn.getNamespace(),
            topic,
            consumerGroup,
            msgFifoExtendLeaseIn.getMsgIds(),
            msgFifoExtendLeaseIn.getLeaseDuration()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.fifo.extend-lease",
            null,
            body,
            MsgFifoExtendLeaseOut.class
        );
    }

    /** Configures retry and DLQ behavior for a fifo consumer group on a topic. */
    public MsgFifoConfigureOut configure(
        String topic,
        String consumerGroup,
        final MsgFifoConfigureIn msgFifoConfigureIn
    ) throws DiomException {
        MsgFifoConfigureIn_ body = new MsgFifoConfigureIn_(
            msgFifoConfigureIn.getNamespace(),
            topic,
            consumerGroup,
            msgFifoConfigureIn.getRetrySchedule(),
            msgFifoConfigureIn.getDlqTopic()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.fifo.configure",
            null,
            body,
            MsgFifoConfigureOut.class
        );
    }

    /** Configures retry and DLQ behavior for a fifo consumer group on a topic. */
    public MsgFifoConfigureOut configure(
        String topic,
        String consumerGroup
    ) throws DiomException {
        return this.configure(
            topic,
            consumerGroup,
            new MsgFifoConfigureIn()
        );
    }

    /** Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ. */
    public MsgFifoNackOut nack(
        String topic,
        String consumerGroup,
        final MsgFifoNackIn msgFifoNackIn
    ) throws DiomException {
        MsgFifoNackIn_ body = new MsgFifoNackIn_(
            msgFifoNackIn.getNamespace(),
            topic,
            consumerGroup,
            msgFifoNackIn.getMsgIds()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.fifo.nack",
            null,
            body,
            MsgFifoNackOut.class
        );
    }

    /** Moves all dead-letter queue messages for a fifo consumer group back for reprocessing. */
    public MsgFifoRedriveDlqOut redriveDlq(
        String topic,
        String consumerGroup,
        final MsgFifoRedriveDlqIn msgFifoRedriveDlqIn
    ) throws DiomException {
        MsgFifoRedriveDlqIn_ body = new MsgFifoRedriveDlqIn_(
            msgFifoRedriveDlqIn.getNamespace(),
            topic,
            consumerGroup
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.fifo.redrive-dlq",
            null,
            body,
            MsgFifoRedriveDlqOut.class
        );
    }

    /** Moves all dead-letter queue messages for a fifo consumer group back for reprocessing. */
    public MsgFifoRedriveDlqOut redriveDlq(
        String topic,
        String consumerGroup
    ) throws DiomException {
        return this.redriveDlq(
            topic,
            consumerGroup,
            new MsgFifoRedriveDlqIn()
        );
    }
}