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
@JsonSerialize(using = IdempotencyStartOutSerializer.class)
@JsonDeserialize(using = IdempotencyStartOutDeserializer.class)
public class IdempotencyStartOut {
    private IdempotencyStartOutConfig data;

    public IdempotencyStartOut data(IdempotencyStartOutConfig data) {
        this.data = data;
        return this;
    }

    public String toJson() throws JsonProcessingException {
        return Utils.getObjectMapper().writeValueAsString(this);
    }

    public static IdempotencyStartOut fromJson(String jsonString) throws JsonProcessingException {
        return Utils.getObjectMapper().readValue(jsonString, IdempotencyStartOut.class);
    }
}

@Getter
@NoArgsConstructor
class IdempotencyStartOutSurrogate {
    @JsonProperty("status") String status;
    @JsonProperty("data") JsonNode data;

    IdempotencyStartOutSurrogate(IdempotencyStartOut o, String status, JsonNode data ){
        this.status = status;
        this.data = data;
    }
}


class IdempotencyStartOutSerializer extends StdSerializer<IdempotencyStartOut> {
    public IdempotencyStartOutSerializer() {
        this(null);
    }

    public IdempotencyStartOutSerializer(Class<IdempotencyStartOut> t) {
        super(t);
    }

    @Override
    public void serialize(IdempotencyStartOut value, JsonGenerator gen, SerializerProvider provider) throws IOException {
        IdempotencyStartOutSurrogate surrogate = new IdempotencyStartOutSurrogate(value,value.getData().getVariantName(),value.getData().toJsonNode());
        gen.writeObject(surrogate);
    }
}


class IdempotencyStartOutDeserializer extends StdDeserializer<IdempotencyStartOut> {
    public IdempotencyStartOutDeserializer() {
        this(null);
    }

    public IdempotencyStartOutDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public IdempotencyStartOut deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        IdempotencyStartOutSurrogate surrogate = p.getCodec().readValue(p, IdempotencyStartOutSurrogate.class);
        String status = surrogate.getStatus();
        JsonNode data = surrogate.getData();
        IdempotencyStartOutConfig sourceType = IdempotencyStartOutConfig.fromTypeAndConfig(status, data);
        return new IdempotencyStartOut(sourceType);
    }
}



