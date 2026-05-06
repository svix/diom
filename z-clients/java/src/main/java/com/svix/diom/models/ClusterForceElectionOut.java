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
public class ClusterForceElectionOut {
    @JsonProperty("previous_leader_id") private String previousLeaderId;
    @JsonProperty("new_leader_id") private String newLeaderId;
    public ClusterForceElectionOut() {}

    public ClusterForceElectionOut previousLeaderId(String previousLeaderId) {
        this.previousLeaderId = previousLeaderId;
        return this;
    }

    /**
    * Get previousLeaderId
    *
     * @return previousLeaderId
     */
    @javax.annotation.Nullable
    public String getPreviousLeaderId() {
        return previousLeaderId;
    }

    public void setPreviousLeaderId(String previousLeaderId) {
        this.previousLeaderId = previousLeaderId;
    }

    public ClusterForceElectionOut newLeaderId(String newLeaderId) {
        this.newLeaderId = newLeaderId;
        return this;
    }

    /**
    * Get newLeaderId
    *
     * @return newLeaderId
     */
    @javax.annotation.Nullable
    public String getNewLeaderId() {
        return newLeaderId;
    }

    public void setNewLeaderId(String newLeaderId) {
        this.newLeaderId = newLeaderId;
    }

    /**
     * Create an instance of ClusterForceElectionOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClusterForceElectionOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ClusterForceElectionOut
     */
    public static ClusterForceElectionOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClusterForceElectionOut.class);
    }

    /**
     * Convert an instance of ClusterForceElectionOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}