//go:build integration

package diom_test

import (
	"context"
	"net/url"
	"os"
	"testing"

	diom "diom.svix.com/go/diom"
)

func newClient(t *testing.T) *diom.Diom {
	t.Helper()
	token := os.Getenv("DIOM_TOKEN")
	if token == "" {
		t.Fatal("DIOM_TOKEN must be set")
	}
	serverURL := os.Getenv("DIOM_SERVER_URL")
	if serverURL == "" {
		t.Fatal("DIOM_SERVER_URL must be set")
	}
	u, err := url.Parse(serverURL)
	if err != nil {
		t.Fatalf("failed to parse DIOM_SERVER_URL: %v", err)
	}
	client, err := diom.New(token, &diom.DiomOptions{ServerUrl: u})
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
	_, err := client.Kv().Set(ctx, key, value, diom.KvSetIn{})
	if err != nil {
		t.Fatalf("kv set failed: %v", err)
	}

	// Get
	getResp, err := client.Kv().Get(ctx, key, diom.KvGetIn{})
	if err != nil {
		t.Fatalf("kv get failed: %v", err)
	}
	if string(getResp.Value) != string(value) {
		t.Fatalf("kv get: expected %q, got %q", value, getResp.Value)
	}

	// Delete
	delResp, err := client.Kv().Delete(ctx, key, diom.KvDeleteIn{})
	if err != nil {
		t.Fatalf("kv delete failed: %v", err)
	}
	if !delResp.Success {
		t.Fatal("kv delete: expected success=true")
	}

	// Verify deleted
	getResp2, err := client.Kv().Get(ctx, key, diom.KvGetIn{})
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
	_, err := client.Cache().Set(ctx, key, value, diom.CacheSetIn{Ttl: diom.DurationMs(60000)})
	if err != nil {
		t.Fatalf("cache set failed: %v", err)
	}

	// Get
	getResp, err := client.Cache().Get(ctx, key, diom.CacheGetIn{})
	if err != nil {
		t.Fatalf("cache get failed: %v", err)
	}
	if string(getResp.Value) != string(value) {
		t.Fatalf("cache get: expected %q, got %q", value, getResp.Value)
	}

	// Delete
	delResp, err := client.Cache().Delete(ctx, key, diom.CacheDeleteIn{})
	if err != nil {
		t.Fatalf("cache delete failed: %v", err)
	}
	if !delResp.Success {
		t.Fatal("cache delete: expected success=true")
	}

	// Verify deleted
	getResp2, err := client.Cache().Get(ctx, key, diom.CacheGetIn{})
	if err != nil {
		t.Fatalf("cache get after delete failed: %v", err)
	}
	if getResp2.Value != nil {
		t.Fatalf("cache get after delete: expected nil, got %q", getResp2.Value)
	}
}
