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
    ) throws DiomException {
        MsgNamespaceConfigureIn_ body = new MsgNamespaceConfigureIn_(
            name,
            msgNamespaceConfigureIn.getRetention()
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.namespace.configure",
            null,
            body,
            MsgNamespaceConfigureOut.class
        );
    }

    /** Configures a msgs namespace with the given name. */
    public MsgNamespaceConfigureOut configure(
        String name
    ) throws DiomException {
        return this.configure(
            name,
            new MsgNamespaceConfigureIn()
        );
    }

    /** Gets a msgs namespace by name. */
    public MsgNamespaceGetOut get(
        String name,
        final MsgNamespaceGetIn msgNamespaceGetIn
    ) throws DiomException {
        MsgNamespaceGetIn_ body = new MsgNamespaceGetIn_(
            name
        );

        return this.client.executeRequest(
            "POST",
            "/api/v1.msgs.namespace.get",
            null,
            body,
            MsgNamespaceGetOut.class
        );
    }

    /** Gets a msgs namespace by name. */
    public MsgNamespaceGetOut get(
        String name
    ) throws DiomException {
        return this.get(
            name,
            new MsgNamespaceGetIn()
        );
    }
}