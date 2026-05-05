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
import com.svix.diom.models.MsgTopicConfigureIn;
import com.svix.diom.models.MsgTopicConfigureOut;
import com.svix.diom.models.MsgTopicConfigureIn_;

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
}