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
import com.svix.diom.models.ListResponseMsgTopicListOut;
import com.svix.diom.models.MsgTopicConfigureIn;
import com.svix.diom.models.MsgTopicConfigureOut;
import com.svix.diom.models.MsgTopicDescribeIn;
import com.svix.diom.models.MsgTopicDescribeOut;
import com.svix.diom.models.MsgTopicListIn;
import com.svix.diom.models.MsgTopicConfigureIn_;
import com.svix.diom.models.MsgTopicDescribeIn_;

public class MsgsTopic {
    private final HttpClient client;

    public MsgsTopic(HttpClient client) {
        this.client = client;
    }

    /**
* Configures the number of partitions for a topic.
* 
* Partition count can only be increased, never decreased. The default for a new topic is 1.
*/
    public MsgTopicConfigureOut configure(
        String topic,
        final MsgTopicConfigureIn msgTopicConfigureIn
    ) throws DiomException {
        MsgTopicConfigureIn_ body = new MsgTopicConfigureIn_(
            msgTopicConfigureIn.getNamespace(),
            topic,
            msgTopicConfigureIn.getPartitions()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.topic.configure",
            null,
            body,
            MsgTopicConfigureOut.class
        );
    }

    /** List available topics in the given namespace */
    public ListResponseMsgTopicListOut list(
        final MsgTopicListIn msgTopicListIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.topic.list",
            null,
            msgTopicListIn,
            ListResponseMsgTopicListOut.class
        );
    }

    /** List available topics in the given namespace */
    public ListResponseMsgTopicListOut list(
        
    ) throws DiomException {
        return this.list(
            new MsgTopicListIn()
        );
    }

    /** Show information about the given topic */
    public MsgTopicDescribeOut describe(
        String topic,
        final MsgTopicDescribeIn msgTopicDescribeIn
    ) throws DiomException {
        MsgTopicDescribeIn_ body = new MsgTopicDescribeIn_(
            msgTopicDescribeIn.getNamespace(),
            topic,
            msgTopicDescribeIn.getConsistency()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.topic.describe",
            null,
            body,
            MsgTopicDescribeOut.class
        );
    }

    /** Show information about the given topic */
    public MsgTopicDescribeOut describe(
        String topic
    ) throws DiomException {
        return this.describe(
            topic,
            new MsgTopicDescribeIn()
        );
    }
}