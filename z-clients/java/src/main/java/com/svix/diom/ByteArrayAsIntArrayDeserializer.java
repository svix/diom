package com.svix.diom;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import java.io.ByteArrayOutputStream;
import java.io.IOException;

public class ByteArrayAsIntArrayDeserializer extends JsonDeserializer<byte[]> {
    @Override
    public byte[] deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        if (p.currentToken() != JsonToken.START_ARRAY) {
            return ctxt.reportInputMismatch(byte[].class, "Expected array of integers for byte[]");
        }
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        while (p.nextToken() != JsonToken.END_ARRAY) {
            out.write(p.getIntValue());
        }
        return out.toByteArray();
    }
}
