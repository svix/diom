import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { Diom } from "../dist/index.mjs";

const TOKEN = process.env.DIOM_TOKEN;
const SERVER_URL = process.env.DIOM_SERVER_URL;

if (!TOKEN || !SERVER_URL) {
  throw new Error("DIOM_TOKEN and DIOM_SERVER_URL environment variables must be set");
}

function makeClient() {
  return new Diom(TOKEN, { serverUrl: SERVER_URL });
}

function toBytes(str) {
  return Array.from(Buffer.from(str));
}

describe("SDK Integration Tests", () => {
  it("test_health_ping", async () => {
    const client = makeClient();
    const resp = await client.health.ping();
    assert.ok(resp);
  });

  it("test_kv_set_get_delete", async () => {
    const client = makeClient();
    const key = "js-integration-kv-key";
    const value = toBytes("js-integration-kv-value");

    // Set
    const setResp = await client.kv.set(key, { value });
    assert.strictEqual(setResp.success, true);

    // Get
    const getResp = await client.kv.get(key, {});
    assert.deepStrictEqual(Array.from(getResp.value), value);

    // Delete
    const delResp = await client.kv.delete(key, {});
    assert.strictEqual(delResp.success, true);

    // Verify deleted
    const getResp2 = await client.kv.get(key, {});
    assert.strictEqual(getResp2.value, null);
  });

  it("test_cache_set_get_delete", async () => {
    const client = makeClient();
    const key = "js-integration-cache-key";
    const value = toBytes("js-integration-cache-value");

    // Set
    await client.cache.set(key, { value, ttlMs: 60000 });

    // Get
    const getResp = await client.cache.get(key, {});
    assert.deepStrictEqual(Array.from(getResp.value), value);

    // Delete
    const delResp = await client.cache.delete(key, {});
    assert.strictEqual(delResp.success, true);

    // Verify deleted
    const getResp2 = await client.cache.get(key, {});
    assert.strictEqual(getResp2.value, null);
  });
});
