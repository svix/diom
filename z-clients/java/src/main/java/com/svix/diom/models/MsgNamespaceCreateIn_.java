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
import com.svix.diom.Utils;
import java.time.Duration;
import java.util.Map;
import java.util.Set;
import java.util.List;
import java.util.Optional;
import java.util.HashMap;
import java.time.OffsetDateTime;
import java.util.LinkedHashSet;
import java.util.ArrayList;
import java.util.Objects;

@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class MsgNamespaceCreateIn_ {
    @JsonProperty private String name;
    @JsonProperty private Retention retention;

    public MsgNamespaceCreateIn_(
        String name,
        Retention retention
    ) {
        this.name = name;
        this.retention = retention;
    }

    /**
     * Create an instance of MsgNamespaceCreateIn_ given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of MsgNamespaceCreateIn_
     * @throws JsonProcessingException if the JSON string is invalid with respect to MsgNamespaceCreateIn_
     */
    public static MsgNamespaceCreateIn_ fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, MsgNamespaceCreateIn_.class);
    }

    /**
     * Convert an instance of MsgNamespaceCreateIn_ to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}
