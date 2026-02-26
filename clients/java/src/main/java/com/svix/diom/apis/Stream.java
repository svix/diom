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
import com.svix.diom.models.Ack;
import com.svix.diom.models.AckMsgRangeIn;
import com.svix.diom.models.AckMsgRangeOut;
import com.svix.diom.models.AckOut;
import com.svix.diom.models.AppendToStreamIn;
import com.svix.diom.models.AppendToStreamOut;
import com.svix.diom.models.DlqIn;
import com.svix.diom.models.DlqOut;
import com.svix.diom.models.FetchFromStreamIn;
import com.svix.diom.models.FetchFromStreamOut;
import com.svix.diom.models.RedriveIn;
import com.svix.diom.models.RedriveOut;

public class Stream {
    private final HttpClient client;

    public Stream(HttpClient client) {
        this.client = client;
    }

    /** Appends messages to the stream. */
    public AppendToStreamOut append(
        final AppendToStreamIn appendToStreamIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/append");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            appendToStreamIn,
            AppendToStreamOut.class
            );
    }

    /**
* Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
* 
* Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
* messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
* until the visibility timeout expires, or the messages are acked.
*/
    public FetchFromStreamOut fetch(
        final FetchFromStreamIn fetchFromStreamIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/fetch");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            fetchFromStreamIn,
            FetchFromStreamOut.class
            );
    }

    /**
* Fetches messages from the stream, locking over the consumer group.
* 
* This call prevents other consumers within the same consumer group from reading from the stream
* until either the visibility timeout expires, or the last message in the batch is acknowledged.
*/
    public FetchFromStreamOut fetchLocking(
        final FetchFromStreamIn fetchFromStreamIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/fetch-locking");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            fetchFromStreamIn,
            FetchFromStreamOut.class
            );
    }

    /** Acks the messages for the consumer group, allowing more messages to be consumed. */
    public AckMsgRangeOut ackRange(
        final AckMsgRangeIn ackMsgRangeIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/ack-range");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            ackMsgRangeIn,
            AckMsgRangeOut.class
            );
    }

    /** Acks a single message. */
    public AckOut ack(
        final Ack ack
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/ack");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            ack,
            AckOut.class
            );
    }

    /** Moves a message to the dead letter queue. */
    public DlqOut dlq(
        final DlqIn dlqIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/dlq");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            dlqIn,
            DlqOut.class
            );
    }

    /** Redrives messages from the dead letter queue back to the stream. */
    public RedriveOut redrive(
        final RedriveIn redriveIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/redrive-dlq");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            redriveIn,
            RedriveOut.class
            );
    }
}