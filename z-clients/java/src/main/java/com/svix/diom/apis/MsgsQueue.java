// this file is @generated
package com.svix.diom.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.diom.ApiException;
import com.svix.diom.HttpClient;
import com.svix.diom.Utils;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.MsgQueueAckIn;
import com.svix.diom.models.MsgQueueAckOut;
import com.svix.diom.models.MsgQueueConfigureIn;
import com.svix.diom.models.MsgQueueConfigureOut;
import com.svix.diom.models.MsgQueueNackIn;
import com.svix.diom.models.MsgQueueNackOut;
import com.svix.diom.models.MsgQueueReceiveIn;
import com.svix.diom.models.MsgQueueReceiveOut;
import com.svix.diom.models.MsgQueueRedriveDlqIn;
import com.svix.diom.models.MsgQueueRedriveDlqOut;
import com.svix.diom.models.MsgQueueReceiveIn_;
import com.svix.diom.models.MsgQueueAckIn_;
import com.svix.diom.models.MsgQueueConfigureIn_;
import com.svix.diom.models.MsgQueueNackIn_;
import com.svix.diom.models.MsgQueueRedriveDlqIn_;

public class MsgsQueue {
    private final HttpClient client;

    public MsgsQueue(HttpClient client) {
        this.client = client;
    }

    /**
* Receives messages from a topic as competing consumers.
* 
* Messages are individually leased for the specified duration. Multiple consumers can receive
* different messages from the same topic concurrently. Leased messages are skipped until they
* are acked or their lease expires.
*/
    public MsgQueueReceiveOut receive(
        String topic,
        String consumerGroup,
        final MsgQueueReceiveIn msgQueueReceiveIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/receive");
        MsgQueueReceiveIn_ body = new MsgQueueReceiveIn_(
            msgQueueReceiveIn.getNamespace(),
            topic,
            consumerGroup,
            msgQueueReceiveIn.getBatchSize(),
            msgQueueReceiveIn.getLeaseDurationMs()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgQueueReceiveOut.class
        );
    }

    /**
* Receives messages from a topic as competing consumers.
* 
* Messages are individually leased for the specified duration. Multiple consumers can receive
* different messages from the same topic concurrently. Leased messages are skipped until they
* are acked or their lease expires.
*/
    public MsgQueueReceiveOut receive(
        String topic,
        String consumerGroup
    ) throws IOException, ApiException {
        return this.receive(
            topic,
            consumerGroup,
            new MsgQueueReceiveIn()
        );
    }

    /**
* Acknowledges messages by their opaque msg_ids.
* 
* Acked messages are permanently removed from the queue and will never be re-delivered.
*/
    public MsgQueueAckOut ack(
        String topic,
        String consumerGroup,
        final MsgQueueAckIn msgQueueAckIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/ack");
        MsgQueueAckIn_ body = new MsgQueueAckIn_(
            msgQueueAckIn.getNamespace(),
            topic,
            consumerGroup,
            msgQueueAckIn.getMsgIds()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgQueueAckOut.class
        );
    }

    /**
* Configures retry and DLQ behavior for a consumer group on a topic.
* 
* `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
* the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
*/
    public MsgQueueConfigureOut configure(
        String topic,
        String consumerGroup,
        final MsgQueueConfigureIn msgQueueConfigureIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/configure");
        MsgQueueConfigureIn_ body = new MsgQueueConfigureIn_(
            msgQueueConfigureIn.getNamespace(),
            topic,
            consumerGroup,
            msgQueueConfigureIn.getRetrySchedule(),
            msgQueueConfigureIn.getDlqTopic()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgQueueConfigureOut.class
        );
    }

    /**
* Configures retry and DLQ behavior for a consumer group on a topic.
* 
* `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
* the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
*/
    public MsgQueueConfigureOut configure(
        String topic,
        String consumerGroup
    ) throws IOException, ApiException {
        return this.configure(
            topic,
            consumerGroup,
            new MsgQueueConfigureIn()
        );
    }

    /**
* Rejects messages, sending them to the dead-letter queue.
* 
* Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
* move them back to the queue for reprocessing.
*/
    public MsgQueueNackOut nack(
        String topic,
        String consumerGroup,
        final MsgQueueNackIn msgQueueNackIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/nack");
        MsgQueueNackIn_ body = new MsgQueueNackIn_(
            msgQueueNackIn.getNamespace(),
            topic,
            consumerGroup,
            msgQueueNackIn.getMsgIds()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgQueueNackOut.class
        );
    }

    /** Moves all dead-letter queue messages back to the main queue for reprocessing. */
    public MsgQueueRedriveDlqOut redriveDlq(
        String topic,
        String consumerGroup,
        final MsgQueueRedriveDlqIn msgQueueRedriveDlqIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/redrive-dlq");
        MsgQueueRedriveDlqIn_ body = new MsgQueueRedriveDlqIn_(
            msgQueueRedriveDlqIn.getNamespace(),
            topic,
            consumerGroup
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgQueueRedriveDlqOut.class
        );
    }

    /** Moves all dead-letter queue messages back to the main queue for reprocessing. */
    public MsgQueueRedriveDlqOut redriveDlq(
        String topic,
        String consumerGroup
    ) throws IOException, ApiException {
        return this.redriveDlq(
            topic,
            consumerGroup,
            new MsgQueueRedriveDlqIn()
        );
    }
}