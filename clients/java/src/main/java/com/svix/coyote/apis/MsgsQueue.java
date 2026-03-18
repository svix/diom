// this file is @generated
package com.svix.coyote.apis;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.svix.coyote.ApiException;
import com.svix.coyote.HttpClient;
import com.svix.coyote.Utils;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.coyote.models.MsgQueueAckIn;
import com.svix.coyote.models.MsgQueueAckOut;
import com.svix.coyote.models.MsgQueueConfigureIn;
import com.svix.coyote.models.MsgQueueConfigureOut;
import com.svix.coyote.models.MsgQueueNackIn;
import com.svix.coyote.models.MsgQueueNackOut;
import com.svix.coyote.models.MsgQueueReceiveIn;
import com.svix.coyote.models.MsgQueueReceiveOut;
import com.svix.coyote.models.MsgQueueRedriveDlqIn;
import com.svix.coyote.models.MsgQueueRedriveDlqOut;

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
        final MsgQueueReceiveIn msgQueueReceiveIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/receive");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgQueueReceiveIn,
            MsgQueueReceiveOut.class
        );
    }

    /**
* Acknowledges messages by their opaque msg_ids.
* 
* Acked messages are permanently removed from the queue and will never be re-delivered.
*/
    public MsgQueueAckOut ack(
        final MsgQueueAckIn msgQueueAckIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/ack");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgQueueAckIn,
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
        final MsgQueueConfigureIn msgQueueConfigureIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/configure");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgQueueConfigureIn,
            MsgQueueConfigureOut.class
        );
    }

    /**
* Rejects messages, sending them to the dead-letter queue.
* 
* Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
* move them back to the queue for reprocessing.
*/
    public MsgQueueNackOut nack(
        final MsgQueueNackIn msgQueueNackIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/nack");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgQueueNackIn,
            MsgQueueNackOut.class
        );
    }

    /** Moves all dead-letter queue messages back to the main queue for reprocessing. */
    public MsgQueueRedriveDlqOut redriveDlq(
        final MsgQueueRedriveDlqIn msgQueueRedriveDlqIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/queue/redrive-dlq");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgQueueRedriveDlqIn,
            MsgQueueRedriveDlqOut.class
        );
    }
}