// this file is @generated
package com.svix.diom.models;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.databind.ser.std.StdSerializer;
import com.svix.diom.Utils;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;
import lombok.EqualsAndHashCode;
import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;
import java.io.IOException;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.util.HashMap;
import java.util.Map;
import java.util.List;
import java.util.Objects;
import java.net.URI;
import java.time.OffsetDateTime;

@Setter
@Getter
@ToString
@NoArgsConstructor
@EqualsAndHashCode
@AllArgsConstructor
@JsonSerialize(using = SinkConfigSerializer.class)
@JsonDeserialize(using = SinkConfigDeserializer.class)
public class SinkConfig {
    private SinkConfigConfig data;

    public SinkConfig data(SinkConfigConfig data) {
        this.data = data;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static SinkConfig fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, SinkConfig.class);
    }
}

@Getter
@NoArgsConstructor
class SinkConfigSurrogate {
    @JsonProperty("type") String type;
    @JsonProperty("data") JsonNode data;

    SinkConfigSurrogate(SinkConfig o, String type, JsonNode data ){
        this.type = type;
        this.data = data;
    }
}


class SinkConfigSerializer extends StdSerializer<SinkConfig> {
    public SinkConfigSerializer() {
        this(null);
    }

    public SinkConfigSerializer(Class<SinkConfig> t) {
        super(t);
    }

    @Override
    public void serialize(SinkConfig value, JsonGenerator gen, SerializerProvider provider) throws IOException {
        SinkConfigSurrogate surrogate = new SinkConfigSurrogate(value,value.getData().getVariantName(),value.getData().toJsonNode());
        gen.writeObject(surrogate);
    }
}


class SinkConfigDeserializer extends StdDeserializer<SinkConfig> {
    public SinkConfigDeserializer() {
        this(null);
    }

    public SinkConfigDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public SinkConfig deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        SinkConfigSurrogate surrogate = p.getCodec().readValue(p, SinkConfigSurrogate.class);
        String type = surrogate.getType();
        JsonNode data = surrogate.getData();
        SinkConfigConfig sourceType = SinkConfigConfig.fromTypeAndConfig(type, data);
        return new SinkConfig(sourceType);
    }
}



