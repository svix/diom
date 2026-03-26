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
import com.svix.coyote.models.MsgStreamCommitIn;
import com.svix.coyote.models.MsgStreamCommitOut;
import com.svix.coyote.models.MsgStreamReceiveIn;
import com.svix.coyote.models.MsgStreamReceiveOut;
import com.svix.coyote.models.MsgStreamSeekIn;
import com.svix.coyote.models.MsgStreamSeekOut;
import com.svix.coyote.models.MsgStreamReceiveIn_;
import com.svix.coyote.models.MsgStreamCommitIn_;
import com.svix.coyote.models.MsgStreamSeekIn_;

public class MsgsStream {
    private final HttpClient client;

    public MsgsStream(HttpClient client) {
        this.client = client;
    }

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
    public MsgStreamReceiveOut receive(
        String topic,
        String consumerGroup,
        final MsgStreamReceiveIn msgStreamReceiveIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.stream.receive");
        MsgStreamReceiveIn_ body = new MsgStreamReceiveIn_(
            msgStreamReceiveIn.getNamespace(),
            topic,
            consumerGroup,
            msgStreamReceiveIn.getBatchSize(),
            msgStreamReceiveIn.getLeaseDurationMs(),
            msgStreamReceiveIn.getDefaultStartingPosition(),
            msgStreamReceiveIn.getBatchWaitMs()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgStreamReceiveOut.class
        );
    }

    /**
* Receives messages from a topic using a consumer group.
* 
* Each consumer in the group reads from all partitions. Messages are locked by leases for the
* specified duration to prevent duplicate delivery within the same consumer group.
*/
    public MsgStreamReceiveOut receive(
        String topic,
        String consumerGroup
    ) throws IOException, ApiException {
        return this.receive(
            topic,
            consumerGroup,
            new MsgStreamReceiveIn()
        );
    }

    /**
* Commits an offset for a consumer group on a specific partition.
* 
* The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
* successfully processed offset; future receives will start after it.
*/
    public MsgStreamCommitOut commit(
        String topic,
        String consumerGroup,
        final MsgStreamCommitIn msgStreamCommitIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.stream.commit");
        MsgStreamCommitIn_ body = new MsgStreamCommitIn_(
            msgStreamCommitIn.getNamespace(),
            topic,
            consumerGroup,
            msgStreamCommitIn.getOffset()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgStreamCommitOut.class
        );
    }

    /**
* Repositions a consumer group's read cursor on a topic.
* 
* Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
* partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
* `"latest"` and may be used with or without a partition suffix.
*/
    public MsgStreamSeekOut seek(
        String topic,
        String consumerGroup,
        final MsgStreamSeekIn msgStreamSeekIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.stream.seek");
        MsgStreamSeekIn_ body = new MsgStreamSeekIn_(
            msgStreamSeekIn.getNamespace(),
            topic,
            consumerGroup,
            msgStreamSeekIn.getOffset(),
            msgStreamSeekIn.getPosition()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgStreamSeekOut.class
        );
    }

    /**
* Repositions a consumer group's read cursor on a topic.
* 
* Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
* partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
* `"latest"` and may be used with or without a partition suffix.
*/
    public MsgStreamSeekOut seek(
        String topic,
        String consumerGroup
    ) throws IOException, ApiException {
        return this.seek(
            topic,
            consumerGroup,
            new MsgStreamSeekIn()
        );
    }
}