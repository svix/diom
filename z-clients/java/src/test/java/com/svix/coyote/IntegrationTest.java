package com.svix.coyote;

import static org.junit.jupiter.api.Assertions.*;

import com.svix.coyote.models.*;
import java.time.Duration;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

@Tag("integration")
class IntegrationTest {

    private static final String TOKEN = requireEnv("COYOTE_TOKEN");
    private static final String SERVER_URL = requireEnv("COYOTE_SERVER_URL");

    private static String requireEnv(String name) {
        String value = System.getenv(name);
        if (value == null || value.isEmpty()) {
            throw new IllegalStateException(name + " must be set");
        }
        return value;
    }

    private static Coyote client;

    @BeforeAll
    static void setUp() {
        CoyoteOptions opts = new CoyoteOptions();
        opts.setServerUrl(SERVER_URL);
        client = new Coyote(TOKEN, opts);
    }

    @Test
    void testHealthPing() throws Exception {
        PingOut resp = client.getHealth().ping();
        assertNotNull(resp);
    }

    @Test
    void testKvSetGetDelete() throws Exception {
        String key = "java-integration-kv-key";
        byte[] value = "java-integration-kv-value".getBytes();

        // Set
        KvSetOut setResp = client.getKv().set(key, value);
        assertTrue(setResp.getSuccess());

        // Get
        KvGetOut getResp = client.getKv().get(key);
        assertArrayEquals(value, getResp.getValue());

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
        byte[] value = "java-integration-cache-value".getBytes();

        // Set
        client.getCache().set(key, value, new CacheSetIn().ttl(Duration.ofMillis(60000)));

        // Get
        CacheGetOut getResp = client.getCache().get(key);
        assertArrayEquals(value, getResp.getValue());

        // Delete
        CacheDeleteOut delResp = client.getCache().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        CacheGetOut getResp2 = client.getCache().get(key);
        assertNull(getResp2.getValue());
    }
}
