// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.annotation.JsonFilter;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.svix.diom.DurationMsSerializer;
import com.svix.diom.DurationMsDeserializer;
import com.svix.diom.UnixTimestampMsSerializer;
import com.svix.diom.UnixTimestampMsDeserializer;
import com.svix.diom.Utils;
import java.time.Duration;
import java.time.Instant;
import java.util.Map;
import java.util.Set;
import java.util.List;
import java.util.Optional;
import java.util.HashMap;
import java.time.OffsetDateTime;
import java.util.LinkedHashSet;
import java.util.ArrayList;
import java.net.URI;
import java.util.Objects;
import lombok.EqualsAndHashCode;
import lombok.ToString;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class HttpSinkConfig {
    @JsonProperty private String url;
    @JsonProperty private HttpMethod method;
    @JsonProperty private Map<String, String> headers;
    @JsonProperty private String body;
    public HttpSinkConfig() {}

    public HttpSinkConfig url(String url) {
        this.url = url;
        return this;
    }

    /**
    * Destination URL.
    *
     * @return url
     */
    @javax.annotation.Nonnull
    public String getUrl() {
        return url;
    }

    public void setUrl(String url) {
        this.url = url;
    }

    public HttpSinkConfig method(HttpMethod method) {
        this.method = method;
        return this;
    }

    /**
    * Get method
    *
     * @return method
     */
    @javax.annotation.Nullable
    public HttpMethod getMethod() {
        return method;
    }

    public void setMethod(HttpMethod method) {
        this.method = method;
    }

    public HttpSinkConfig headers(Map<String, String> headers) {
        this.headers = headers;
        return this;
    }

    public HttpSinkConfig putHeadersItem(String key, String headersItem) {
        if (this.headers == null) {
            this.headers = new HashMap<>();
        }
        this.headers.put(key, headersItem);
        return this;
    }
    /**
    * Get headers
    *
     * @return headers
     */
    @javax.annotation.Nullable
    public Map<String, String> getHeaders() {
        return headers;
    }

    public void setHeaders(Map<String, String> headers) {
        this.headers = headers;
    }

    public HttpSinkConfig body(String body) {
        this.body = body;
        return this;
    }

    /**
    * Templated request body. When absent, the raw message value bytes are sent unchanged.
    *
     * @return body
     */
    @javax.annotation.Nullable
    public String getBody() {
        return body;
    }

    public void setBody(String body) {
        this.body = body;
    }

    /**
     * Create an instance of HttpSinkConfig given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of HttpSinkConfig
     * @throws JsonProcessingException if the JSON string is invalid with respect to HttpSinkConfig
     */
    public static HttpSinkConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, HttpSinkConfig.class);
    }

    /**
     * Convert an instance of HttpSinkConfig to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}