package com.svix.diom;

import static org.junit.jupiter.api.Assertions.*;

import com.svix.diom.models.*;
import java.time.Duration;
import java.util.Collections;
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

    @Test
    @Tag("integration")
    void testHealthPing() throws Exception {
        PingOut resp = client.health().ping();
        assertNotNull(resp);
    }

    @Test
    @Tag("integration")
    void testKvSetGetDelete() throws Exception {
        String key = "java-integration-kv-key";
        byte[] value = "java-integration-kv-value".getBytes();

        // Set
        KvSetOut setResp = client.kv().set(key, value);

        // Get
        KvGetOut getResp = client.kv().get(key);
        assertArrayEquals(value, getResp.getValue());

        // Delete
        KvDeleteOut delResp = client.kv().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        KvGetOut getResp2 = client.kv().get(key);
        assertNull(getResp2.getValue());
    }

    @Test
    @Tag("integration")
    void testMsgsQueueSubresourceChaining() throws Exception {
        String namespace = "java-integration-ns";
        String topic = "java-integration-topic";
        String consumerGroup = "java-integration-cg";

        client.msgs().namespace().configure(namespace);
        client.msgs().publish(topic, new MsgPublishIn()
            .namespace(namespace)
            .msgs(Collections.singletonList(new MsgIn().value("hello".getBytes()))));

        MsgQueueReceiveOut received = client.msgs().queue().receive(
            topic, consumerGroup, new MsgQueueReceiveIn().namespace(namespace));
        assertNotNull(received);
        assertFalse(received.getMsgs().isEmpty());

        String msgId = received.getMsgs().get(0).getMsgId();
        MsgQueueAckOut ackResp = client.msgs().queue().ack(
            topic, consumerGroup,
            new MsgQueueAckIn()
                .namespace(namespace)
                .msgIds(Collections.singletonList(msgId)));
        assertNotNull(ackResp);
    }

    @Test
    @Tag("integration")
    void testCacheSetGetDelete() throws Exception {
        String key = "java-integration-cache-key";
        byte[] value = "java-integration-cache-value".getBytes();

        // Set
        client.cache().set(key, value, new CacheSetIn().ttl(Duration.ofMillis(60000)));

        // Get
        CacheGetOut getResp = client.cache().get(key);
        assertArrayEquals(value, getResp.getValue());

        // Delete
        CacheDeleteOut delResp = client.cache().delete(key);
        assertTrue(delResp.getSuccess());

        // Verify deleted
        CacheGetOut getResp2 = client.cache().get(key);
        assertNull(getResp2.getValue());
    }
}
