// this file is @generated
package com.svix.coyote.models;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonAutoDetect;
import com.fasterxml.jackson.annotation.JsonAutoDetect.Visibility;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.annotation.JsonFilter;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.coyote.Utils;
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
public class TransformIn {
    @JsonProperty private String input;
    @JsonProperty private String script;
    @JsonProperty("max_duration_ms") private Long maxDurationMs;
    public TransformIn() {}

    public TransformIn input(String input) {
        this.input = input;
        return this;
    }

    /**
    * JSON-encoded payload passed to the script as `input`.
    *
     * @return input
     */
    @javax.annotation.Nonnull
    public String getInput() {
        return input;
    }

    public void setInput(String input) {
        this.input = input;
    }

    public TransformIn script(String script) {
        this.script = script;
        return this;
    }

    /**
    * JavaScript source. Must define a `handler(input)` function.
    *
     * @return script
     */
    @javax.annotation.Nonnull
    public String getScript() {
        return script;
    }

    public void setScript(String script) {
        this.script = script;
    }

    public TransformIn maxDurationMs(Long maxDurationMs) {
        this.maxDurationMs = maxDurationMs;
        return this;
    }

    /**
    * How long to let the script run before being killed.
    *
     * @return maxDurationMs
     */
    @javax.annotation.Nullable
    public Long getMaxDurationMs() {
        return maxDurationMs;
    }

    public void setMaxDurationMs(Long maxDurationMs) {
        this.maxDurationMs = maxDurationMs;
    }

    /**
     * Create an instance of TransformIn given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of TransformIn
     * @throws JsonProcessingException if the JSON string is invalid with respect to TransformIn
     */
    public static TransformIn fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, TransformIn.class);
    }

    /**
     * Convert an instance of TransformIn to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}