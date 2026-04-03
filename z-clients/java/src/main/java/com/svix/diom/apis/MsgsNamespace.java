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
import java.util.List;
import java.util.Map;
import java.util.Set;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.MsgNamespaceCreateIn;
import com.svix.diom.models.MsgNamespaceCreateOut;
import com.svix.diom.models.MsgNamespaceGetIn;
import com.svix.diom.models.MsgNamespaceGetOut;
import com.svix.diom.models.MsgNamespaceCreateIn_;
import com.svix.diom.models.MsgNamespaceGetIn_;

public class MsgsNamespace {
    private final HttpClient client;

    public MsgsNamespace(HttpClient client) {
        this.client = client;
    }

    /** Creates or updates a msgs namespace with the given name. */
    public MsgNamespaceCreateOut create(
        String name,
        final MsgNamespaceCreateIn msgNamespaceCreateIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.namespace.create");
        MsgNamespaceCreateIn_ body = new MsgNamespaceCreateIn_(
            name,
            msgNamespaceCreateIn.getRetention()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgNamespaceCreateOut.class
        );
    }

    /** Creates or updates a msgs namespace with the given name. */
    public MsgNamespaceCreateOut create(
        String name
    ) throws IOException, ApiException {
        return this.create(
            name,
            new MsgNamespaceCreateIn()
        );
    }

    /** Gets a msgs namespace by name. */
    public MsgNamespaceGetOut get(
        String name,
        final MsgNamespaceGetIn msgNamespaceGetIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.namespace.get");
        MsgNamespaceGetIn_ body = new MsgNamespaceGetIn_(
            name
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgNamespaceGetOut.class
        );
    }

    /** Gets a msgs namespace by name. */
    public MsgNamespaceGetOut get(
        String name
    ) throws IOException, ApiException {
        return this.get(
            name,
            new MsgNamespaceGetIn()
        );
    }
}