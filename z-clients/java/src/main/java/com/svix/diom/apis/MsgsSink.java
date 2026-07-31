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
import com.svix.diom.models.ListResponseSinkOut;
import com.svix.diom.models.SinkConfigureIn;
import com.svix.diom.models.SinkConfigureOut;
import com.svix.diom.models.SinkDeleteIn;
import com.svix.diom.models.SinkDeleteOut;
import com.svix.diom.models.SinkListIn;
import com.svix.diom.models.SinkListIn_;

public class MsgsSink {
    private final HttpClient client;

    public MsgsSink(HttpClient client) {
        this.client = client;
    }

    /** Create or update a sink for a topic. Overwrites any existing sink with the same id. */
    public SinkConfigureOut configure(
        final SinkConfigureIn sinkConfigureIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.sink.configure",
            null,
            sinkConfigureIn,
            SinkConfigureOut.class
        );
    }

    /** Delete a sink. */
    public SinkDeleteOut delete(
        final SinkDeleteIn sinkDeleteIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.sink.delete",
            null,
            sinkDeleteIn,
            SinkDeleteOut.class
        );
    }

    /** List sink configurations for a topic. */
    public ListResponseSinkOut list(
        String topic,
        final SinkListIn sinkListIn
    ) throws DiomException {
        SinkListIn_ body = new SinkListIn_(
            sinkListIn.getNamespace(),
            topic,
            sinkListIn.getLimit(),
            sinkListIn.getIterator()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.sink.list",
            null,
            body,
            ListResponseSinkOut.class
        );
    }

    /** List sink configurations for a topic. */
    public ListResponseSinkOut list(
        String topic
    ) throws DiomException {
        return this.list(
            topic,
            new SinkListIn()
        );
    }
}