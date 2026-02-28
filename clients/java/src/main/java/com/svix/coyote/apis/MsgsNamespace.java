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
import com.svix.coyote.models.MsgNamespaceCreateIn;
import com.svix.coyote.models.MsgNamespaceCreateOut;
import com.svix.coyote.models.MsgNamespaceGetIn;
import com.svix.coyote.models.MsgNamespaceGetOut;

public class MsgsNamespace {
    private final HttpClient client;

    public MsgsNamespace(HttpClient client) {
        this.client = client;
    }

    /** Creates or updates a msgs namespace with the given name. */
    public MsgNamespaceCreateOut create(
        final MsgNamespaceCreateIn msgNamespaceCreateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/namespace/create");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgNamespaceCreateIn,
            MsgNamespaceCreateOut.class
            );
    }

    /** Gets a msgs namespace by name. */
    public MsgNamespaceGetOut get(
        final MsgNamespaceGetIn msgNamespaceGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/msgs/namespace/get");
        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            msgNamespaceGetIn,
            MsgNamespaceGetOut.class
            );
    }
}