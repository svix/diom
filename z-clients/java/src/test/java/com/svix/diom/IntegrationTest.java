package com.svix.diom;

import static org.junit.jupiter.api.Assertions.*;

import com.svix.diom.models.*;
import java.time.Duration;
import java.util.Arrays;
import java.util.List;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

@Tag("integration")
class IntegrationTest {

    private static final String TOKEN = requireEnv("DIOM_TOKEN");
    private static final String SERVER_URL = requireEnv("DIOM_SERVER_URL");

    private static String requireEnv(String name) {
        String value = System.getenv(name);
        if (value == null || value.isEmpty()) {
            throw new IllegalStateException(name + " must be set");
        }
        return value;
    }

    private static Diom client;

    @BeforeAll
    static void setUp() {
        DiomOptions opts = new DiomOptions();
        opts.setServerUrl(SERVER_URL);
        client = new Diom(TOKEN, opts);
    }

    private static List<Byte> toBytes(String s) {
        byte[] raw = s.getBytes();
        Byte[] boxed = new Byte[raw.length];
        for (int i = 0; i < raw.length; i++) {
            boxed[i] = raw[i];
        }
        return Arrays.asList(boxed);
    }

    @Test
    void testHealthPing() throws Exception {
        PingOut resp = client.getHealth().ping();
        assertNotNull(resp);
    }

    @Test
    void testKvSetGetDelete() throws Exception {
        String key = "java-integration-kv-key";
        List<Byte> value = toBytes("java-integration-kv-value");

        // Set
        KvSetOut setResp = client.getKv().set(key, value);
        assertTrue(setResp.getSuccess());

        // Get
        KvGetOut getResp = client.getKv().get(key);
        assertEquals(value, getResp.getValue());

        // Delete
        KvDeleteOut delResp = client.getKv().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        KvGetOut getResp2 = client.getKv().get(key);
        assertNull(getResp2.getValue());
    }

    @Test
    void testCacheSetGetDelete() throws Exception {
        String key = "java-integration-cache-key";
        List<Byte> value = toBytes("java-integration-cache-value");

        // Set
        client.getCache().set(key, value, new CacheSetIn().ttl(Duration.ofMillis(60000)));

        // Get
        CacheGetOut getResp = client.getCache().get(key);
        assertEquals(value, getResp.getValue());

        // Delete
        CacheDeleteOut delResp = client.getCache().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        CacheGetOut getResp2 = client.getCache().get(key);
        assertNull(getResp2.getValue());
    }
}
