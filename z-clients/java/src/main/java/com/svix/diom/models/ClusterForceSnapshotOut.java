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
import com.svix.diom.ByteArrayAsIntArraySerializer;
import com.svix.diom.ByteArrayAsIntArrayDeserializer;
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
import java.net.URI;
import java.util.Objects;
import lombok.EqualsAndHashCode;
import lombok.ToString;

@ToString
@EqualsAndHashCode
@JsonInclude(JsonInclude.Include.NON_NULL)
@JsonAutoDetect(getterVisibility = Visibility.NONE, setterVisibility = Visibility.NONE)
public class ClusterForceSnapshotOut {
    @JsonProperty("snapshot_time") private OffsetDateTime snapshotTime;
    @JsonProperty("snapshot_log_index") private Long snapshotLogIndex;
    @JsonProperty("snapshot_id") private String snapshotId;
    public ClusterForceSnapshotOut() {}

    public ClusterForceSnapshotOut snapshotTime(OffsetDateTime snapshotTime) {
        this.snapshotTime = snapshotTime;
        return this;
    }

    /**
    * The wall-clock time at which the snapshot was initiated
    *
     * @return snapshotTime
     */
    @javax.annotation.Nonnull
    public OffsetDateTime getSnapshotTime() {
        return snapshotTime;
    }

    public void setSnapshotTime(OffsetDateTime snapshotTime) {
        this.snapshotTime = snapshotTime;
    }

    public ClusterForceSnapshotOut snapshotLogIndex(Long snapshotLogIndex) {
        this.snapshotLogIndex = snapshotLogIndex;
        return this;
    }

    /**
    * The log index at which the snapshot was initiated
    *
     * @return snapshotLogIndex
     */
    @javax.annotation.Nonnull
    public Long getSnapshotLogIndex() {
        return snapshotLogIndex;
    }

    public void setSnapshotLogIndex(Long snapshotLogIndex) {
        this.snapshotLogIndex = snapshotLogIndex;
    }

    public ClusterForceSnapshotOut snapshotId(String snapshotId) {
        this.snapshotId = snapshotId;
        return this;
    }

    /**
    * If this is `null`, the snapshot is still building in the background
    *
     * @return snapshotId
     */
    @javax.annotation.Nullable
    public String getSnapshotId() {
        return snapshotId;
    }

    public void setSnapshotId(String snapshotId) {
        this.snapshotId = snapshotId;
    }

    /**
     * Create an instance of ClusterForceSnapshotOut given a JSON string
     *
     * @param jsonString JSON string
     * @return An instance of ClusterForceSnapshotOut
     * @throws JsonProcessingException if the JSON string is invalid with respect to ClusterForceSnapshotOut
     */
    public static ClusterForceSnapshotOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, ClusterForceSnapshotOut.class);
    }

    /**
     * Convert an instance of ClusterForceSnapshotOut to a JSON string
     *
     * @return JSON string
     */
    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }
}