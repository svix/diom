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
import com.svix.diom.models.MsgStreamCancelLeaseIn;
import com.svix.diom.models.MsgStreamCancelLeaseOut;
import com.svix.diom.models.MsgStreamCommitIn;
import com.svix.diom.models.MsgStreamCommitOut;
import com.svix.diom.models.MsgStreamReceiveIn;
import com.svix.diom.models.MsgStreamReceiveOut;
import com.svix.diom.models.MsgStreamSeekIn;
import com.svix.diom.models.MsgStreamSeekOut;
import com.svix.diom.models.MsgStreamReceiveIn_;
import com.svix.diom.models.MsgStreamCommitIn_;
import com.svix.diom.models.MsgStreamSeekIn_;
import com.svix.diom.models.MsgStreamCancelLeaseIn_;

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
    ) throws DiomException {
        MsgStreamReceiveIn_ body = new MsgStreamReceiveIn_(
            msgStreamReceiveIn.getNamespace(),
            topic,
            consumerGroup,
            msgStreamReceiveIn.getBatchSize(),
            msgStreamReceiveIn.getLeaseDuration(),
            msgStreamReceiveIn.getDefaultStartingPosition(),
            msgStreamReceiveIn.getBatchWait()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.stream.receive",
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
    ) throws DiomException {
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
    ) throws DiomException {
        MsgStreamCommitIn_ body = new MsgStreamCommitIn_(
            msgStreamCommitIn.getNamespace(),
            topic,
            consumerGroup,
            msgStreamCommitIn.getOffset()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.stream.commit",
            null,
            body,
            MsgStreamCommitOut.class
        );
    }

    /**
* Repositions a consumer group's read cursor on a topic.
* 
* Provide exactly one of `offset`, `position`, or `timestamp`. When using `offset`, the topic
* must include a partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts
* `"earliest"` or `"latest"` and may be used with or without a partition suffix. The `timestamp`
* field accepts a Unix timestamp in milliseconds and seeks to the first message at or after that
* time.
*/
    public MsgStreamSeekOut seek(
        String topic,
        String consumerGroup,
        final MsgStreamSeekIn msgStreamSeekIn
    ) throws DiomException {
        MsgStreamSeekIn_ body = new MsgStreamSeekIn_(
            msgStreamSeekIn.getNamespace(),
            topic,
            consumerGroup,
            msgStreamSeekIn.getOffset(),
            msgStreamSeekIn.getPosition(),
            msgStreamSeekIn.getTimestamp()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.stream.seek",
            null,
            body,
            MsgStreamSeekOut.class
        );
    }

    /**
* Repositions a consumer group's read cursor on a topic.
* 
* Provide exactly one of `offset`, `position`, or `timestamp`. When using `offset`, the topic
* must include a partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts
* `"earliest"` or `"latest"` and may be used with or without a partition suffix. The `timestamp`
* field accepts a Unix timestamp in milliseconds and seeks to the first message at or after that
* time.
*/
    public MsgStreamSeekOut seek(
        String topic,
        String consumerGroup
    ) throws DiomException {
        return this.seek(
            topic,
            consumerGroup,
            new MsgStreamSeekIn()
        );
    }

    /**
* Cancels a current stream lease.
* 
* Used when a consumer cannot process a batch and wants to release it immediately rather than
* wait for lease expiration.
*/
    public MsgStreamCancelLeaseOut cancelLease(
        String topic,
        String consumerGroup,
        final MsgStreamCancelLeaseIn msgStreamCancelLeaseIn
    ) throws DiomException {
        MsgStreamCancelLeaseIn_ body = new MsgStreamCancelLeaseIn_(
            msgStreamCancelLeaseIn.getNamespace(),
            topic,
            consumerGroup
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.stream.cancel-lease",
            null,
            body,
            MsgStreamCancelLeaseOut.class
        );
    }

    /**
* Cancels a current stream lease.
* 
* Used when a consumer cannot process a batch and wants to release it immediately rather than
* wait for lease expiration.
*/
    public MsgStreamCancelLeaseOut cancelLease(
        String topic,
        String consumerGroup
    ) throws DiomException {
        return this.cancelLease(
            topic,
            consumerGroup,
            new MsgStreamCancelLeaseIn()
        );
    }
}