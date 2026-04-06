package com.svix.diom;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;
import java.io.IOException;
import java.time.Duration;

public class DurationMsSerializer extends JsonSerializer<Duration> {
    @Override
    public void serialize(Duration value, JsonGenerator gen, SerializerProvider serializers)
            throws IOException {
        gen.writeNumber(value.toMillis());
    }
}
