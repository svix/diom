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
import com.svix.diom.models.ListResponseSvixPollerOut;
import com.svix.diom.models.SvixPollerCreateIn;
import com.svix.diom.models.SvixPollerCreateOut;
import com.svix.diom.models.SvixPollerDeleteIn;
import com.svix.diom.models.SvixPollerDeleteOut;
import com.svix.diom.models.SvixPollerListIn;
import com.svix.diom.models.SvixPollerListIn_;

public class MsgsSvixPoller {
    private final HttpClient client;

    public MsgsSvixPoller(HttpClient client) {
        this.client = client;
    }

    /** Create a Svix poller configuration for a topic. */
    public SvixPollerCreateOut create(
        final SvixPollerCreateIn svixPollerCreateIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.svix-poller.create",
            null,
            svixPollerCreateIn,
            SvixPollerCreateOut.class
        );
    }

    /** Delete a Svix poller configuration. */
    public SvixPollerDeleteOut delete(
        final SvixPollerDeleteIn svixPollerDeleteIn
    ) throws DiomException {

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.svix-poller.delete",
            null,
            svixPollerDeleteIn,
            SvixPollerDeleteOut.class
        );
    }

    /** List Svix poller configurations for a topic. */
    public ListResponseSvixPollerOut list(
        String topic,
        final SvixPollerListIn svixPollerListIn
    ) throws DiomException {
        SvixPollerListIn_ body = new SvixPollerListIn_(
            svixPollerListIn.getNamespace(),
            topic,
            svixPollerListIn.getLimit(),
            svixPollerListIn.getIterator()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.svix-poller.list",
            null,
            body,
            ListResponseSvixPollerOut.class
        );
    }

    /** List Svix poller configurations for a topic. */
    public ListResponseSvixPollerOut list(
        String topic
    ) throws DiomException {
        return this.list(
            topic,
            new SvixPollerListIn()
        );
    }
}