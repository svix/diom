package diom_test

import (
	"context"
	"io"
	"net"
	"net/http"
	"net/http/httptest"
	"net/url"
	"sync/atomic"
	"testing"
	"time"

	diom "diom.com/go/diom"
	"github.com/vmihailenco/msgpack/v5"
)

func Test_retries_on_5xx(t *testing.T) {
	tests := []struct {
		name           string
		failFirst      bool
		expectedCalls  int32
		expectedStatus int
	}{
		{
			name:           "request succeeds with retry after initial failure",
			failFirst:      true,
			expectedCalls:  2,
			expectedStatus: http.StatusOK,
		},
		{
			name:           "request succeeds immediately without retry",
			failFirst:      false,
			expectedCalls:  1,
			expectedStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var requestCount int32
			var receivedBodies []string

			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				count := atomic.AddInt32(&requestCount, 1)

				body, err := io.ReadAll(r.Body)
				if err != nil {
					t.Errorf("failed to read request body: %v", err)
					return
				}
				receivedBodies = append(receivedBodies, string(body))

				if tt.failFirst && count == 1 {
					w.WriteHeader(http.StatusInternalServerError)
					w.Header().Set("Content-Type", "application/json")
					_, err := w.Write([]byte(`{"error": "server error"}`))
					if err != nil {
						t.Errorf("failed to write status code - error: %v", err)
						return
					}
					return
				}

				w.Header().Set("Content-Type", "application/msgpack")
				w.WriteHeader(tt.expectedStatus)
				response := diom.PingOut{
					Ok: true,
				}
				err = msgpack.NewEncoder(w).Encode(response)
				if err != nil {
					t.Errorf("failed to encode json - error: %v", err)
				}
			}))
			defer server.Close()

			u, err := url.Parse(server.URL)
			if err != nil {
				t.Fatalf("error from mock server")
			}

			client, err := diom.New("...", &diom.DiomOptions{
				ServerUrl:     u,
				RetrySchedule: &[]time.Duration{10 * time.Millisecond},
			})

			ctx := context.Background()
			resp, err := client.Health().Ping(ctx)
			if err != nil {
				t.Fatalf("ping call failed: %s", err)
			}

			if !resp.Ok {
				t.Fatalf("Invalid response")
			}

			// And ... our request count should match the expected number of calls
			if requestCount != tt.expectedCalls {
				t.Errorf("Expected %d requests, got %d", tt.expectedCalls, requestCount)
			}

			// And ... we should have the same number of request bodies
			if len(receivedBodies) != int(tt.expectedCalls) {
				t.Fatalf("Expected %d request bodies, got %d", tt.expectedCalls, len(receivedBodies))
			}
		})
	}
}

func Test_retries_on_timeout(t *testing.T) {
	var requestCount int32
	var receivedBodies []string

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		count := atomic.AddInt32(&requestCount, 1)

		body, err := io.ReadAll(r.Body)
		if err != nil {
			t.Errorf("failed to read request body: %v", err)
			return
		}
		receivedBodies = append(receivedBodies, string(body))

		if count == 1 {
			time.Sleep(150 * time.Millisecond)
		}

		w.Header().Set("Content-Type", "application/msgpack")
		w.WriteHeader(http.StatusOK)
		response := diom.PingOut{
			Ok: true,
		}
		err = msgpack.NewEncoder(w).Encode(response)
		if err != nil {
			t.Errorf("failed to encode json - error: %v", err)
		}
	}))
	defer server.Close()

	u, err := url.Parse(server.URL)
	if err != nil {
		t.Fatalf("error from mock server")
	}

	client, err := diom.New("...", &diom.DiomOptions{
		ServerUrl:     u,
		RetrySchedule: &[]time.Duration{10 * time.Millisecond},
		Timeout:       10 * time.Millisecond,
	})

	ctx := context.Background()
	resp, err := client.Health().Ping(ctx)
	if err != nil {
		t.Fatalf("ping call failed: %s", err)
	}

	if !resp.Ok {
		t.Fatalf("Invalid response")
	}

	// And ... our request count should match the expected number of calls
	if requestCount != 2 {
		t.Errorf("Expected 2 requests, got %d", requestCount)
	}
}

func Test_retries_on_io_error(t *testing.T) {
	// Given ... we have a test server
	var requestCount int32
	var receivedBodies []string

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		atomic.AddInt32(&requestCount, 1)

		body, err := io.ReadAll(r.Body)
		if err != nil {
			t.Errorf("failed to read request body: %v", err)
			return
		}
		receivedBodies = append(receivedBodies, string(body))

		w.Header().Set("Content-Type", "application/msgpack")
		w.WriteHeader(http.StatusOK)
		response := diom.PingOut{
			Ok: true,
		}
		err = msgpack.NewEncoder(w).Encode(response)
		if err != nil {
			t.Errorf("failed to encode json - error: %v", err)
		}
	}))
	defer server.Close()

	u, err := url.Parse(server.URL)
	if err != nil {
		t.Fatalf("error from mock server")
	}

	dialer := &net.Dialer{
		Timeout:   1 * time.Second,
		KeepAlive: 1 * time.Second,
	}
	var dialCount int32
	dialerContext := func(ctx context.Context, network, addr string) (net.Conn, error) {
		atomic.AddInt32(&dialCount, 1)
		if dialCount == 1 {
			return nil, &net.DNSError{
				UnwrapErr:   nil,
				Err:         "no such host",
				Name:        "localhost",
				Server:      "localhost",
				IsTimeout:   false,
				IsTemporary: false,
			}
		}
		return dialer.DialContext(ctx, network, addr)
	}
	tr := http.DefaultTransport.(*http.Transport).Clone()
	tr.DialContext = dialerContext

	httpClient := http.Client{
		Timeout:   15 * time.Second,
		Transport: tr,
	}

	client, err := diom.New("...", &diom.DiomOptions{
		ServerUrl:     u,
		RetrySchedule: &[]time.Duration{10 * time.Millisecond},
		Timeout:       10 * time.Millisecond,
		HTTPClient:    &httpClient,
	})

	ctx := context.Background()
	resp, err := client.Health().Ping(ctx)
	if err != nil {
		t.Fatalf("ping call failed: %s", err)
	}

	if !resp.Ok {
		t.Fatalf("Invalid response")
	}

	// We try the lookup twice
	if dialCount != 2 {
		t.Errorf("Expected 2 dials, got %d", requestCount)
	}

	// Only one request goes through
	if requestCount != 1 {
		t.Errorf("Expected 1 request, got %d", requestCount)
	}
}
