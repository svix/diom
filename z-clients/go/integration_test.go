//go:build integration

package coyote_test

import (
	"context"
	"net/url"
	"os"
	"testing"

	coyote "github.com/svix/coyote/z-clients/go"
)

func newClient(t *testing.T) *coyote.Coyote {
	t.Helper()
	token := os.Getenv("COYOTE_TOKEN")
	if token == "" {
		t.Fatal("COYOTE_TOKEN must be set")
	}
	serverURL := os.Getenv("COYOTE_SERVER_URL")
	if serverURL == "" {
		t.Fatal("COYOTE_SERVER_URL must be set")
	}
	u, err := url.Parse(serverURL)
	if err != nil {
		t.Fatalf("failed to parse COYOTE_SERVER_URL: %v", err)
	}
	client, err := coyote.New(token, &coyote.CoyoteOptions{ServerUrl: u})
	if err != nil {
		t.Fatalf("failed to create client: %v", err)
	}
	return client
}

func TestHealthPing(t *testing.T) {
	client := newClient(t)
	_, err := client.Health().Ping(context.Background())
	if err != nil {
		t.Fatalf("health ping failed: %v", err)
	}
}

func TestKvSetGetDelete(t *testing.T) {
	client := newClient(t)
	ctx := context.Background()
	key := "go-integration-kv-key"
	value := []uint8("go-integration-kv-value")

	// Set
	setResp, err := client.Kv().Set(ctx, key, value, coyote.KvSetIn{})
	if err != nil {
		t.Fatalf("kv set failed: %v", err)
	}
	if !setResp.Success {
		t.Fatal("kv set: expected success=true")
	}

	// Get
	getResp, err := client.Kv().Get(ctx, key, coyote.KvGetIn{})
	if err != nil {
		t.Fatalf("kv get failed: %v", err)
	}
	if string(getResp.Value) != string(value) {
		t.Fatalf("kv get: expected %q, got %q", value, getResp.Value)
	}

	// Delete
	delResp, err := client.Kv().Delete(ctx, key, coyote.KvDeleteIn{})
	if err != nil {
		t.Fatalf("kv delete failed: %v", err)
	}
	if !delResp.Success {
		t.Fatal("kv delete: expected success=true")
	}

	// Verify deleted
	getResp2, err := client.Kv().Get(ctx, key, coyote.KvGetIn{})
	if err != nil {
		t.Fatalf("kv get after delete failed: %v", err)
	}
	if getResp2.Value != nil {
		t.Fatalf("kv get after delete: expected nil, got %q", getResp2.Value)
	}
}

func TestCacheSetGetDelete(t *testing.T) {
	client := newClient(t)
	ctx := context.Background()
	key := "go-integration-cache-key"
	value := []uint8("go-integration-cache-value")

	// Set
	_, err := client.Cache().Set(ctx, key, value, coyote.CacheSetIn{Ttl: coyote.DurationMs(60000)})
	if err != nil {
		t.Fatalf("cache set failed: %v", err)
	}

	// Get
	getResp, err := client.Cache().Get(ctx, key, coyote.CacheGetIn{})
	if err != nil {
		t.Fatalf("cache get failed: %v", err)
	}
	if string(getResp.Value) != string(value) {
		t.Fatalf("cache get: expected %q, got %q", value, getResp.Value)
	}

	// Delete
	delResp, err := client.Cache().Delete(ctx, key, coyote.CacheDeleteIn{})
	if err != nil {
		t.Fatalf("cache delete failed: %v", err)
	}
	if !delResp.Success {
		t.Fatal("cache delete: expected success=true")
	}

	// Verify deleted
	getResp2, err := client.Cache().Get(ctx, key, coyote.CacheGetIn{})
	if err != nil {
		t.Fatalf("cache get after delete failed: %v", err)
	}
	if getResp2.Value != nil {
		t.Fatalf("cache get after delete: expected nil, got %q", getResp2.Value)
	}
}
