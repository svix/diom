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
import java.util.Map;
import java.util.Set;
import lombok.Getter;
import okhttp3.Headers;
import okhttp3.HttpUrl;
import com.svix.diom.models.CreateNamespaceIn;
import com.svix.diom.models.CreateNamespaceOut;
import com.svix.diom.models.GetNamespaceIn;
import com.svix.diom.models.GetNamespaceOut;

public class Msgs {
    private final HttpClient client;

    public Msgs(HttpClient client) {
        this.client = client;
    }
}