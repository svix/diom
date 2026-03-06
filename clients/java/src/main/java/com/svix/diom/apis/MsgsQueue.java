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
import com.svix.diom.models.MsgQueueReceiveIn;
import com.svix.diom.models.MsgQueueReceiveOut;

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
}