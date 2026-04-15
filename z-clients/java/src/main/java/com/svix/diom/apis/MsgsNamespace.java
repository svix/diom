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
import com.svix.diom.models.MsgNamespaceConfigureIn;
import com.svix.diom.models.MsgNamespaceConfigureOut;
import com.svix.diom.models.MsgNamespaceGetIn;
import com.svix.diom.models.MsgNamespaceGetOut;
import com.svix.diom.models.MsgNamespaceConfigureIn_;
import com.svix.diom.models.MsgNamespaceGetIn_;

public class MsgsNamespace {
    private final HttpClient client;

    public MsgsNamespace(HttpClient client) {
        this.client = client;
    }

    /** Configures a msgs namespace with the given name. */
    public MsgNamespaceConfigureOut configure(
        String name,
        final MsgNamespaceConfigureIn msgNamespaceConfigureIn
    ) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1.msgs.namespace.configure");
        MsgNamespaceConfigureIn_ body = new MsgNamespaceConfigureIn_(
            name,
            msgNamespaceConfigureIn.getRetention()
        );

        return this.client.executeRequest(
            "POST",
            url.build(),
            null,
            body,
            MsgNamespaceConfigureOut.class
        );
    }

    /** Configures a msgs namespace with the given name. */
    public MsgNamespaceConfigureOut configure(
        String name
    ) throws IOException, ApiException {
        return this.configure(
            name,
            new MsgNamespaceConfigureIn()
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