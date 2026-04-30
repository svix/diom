package diom_proto

import (
	"bytes"
	"context"
	"crypto/tls"
	"io"
	"log"
	"math/rand"
	"net/http"
	"net/http/httputil"
	"strconv"
	"time"

	"github.com/vmihailenco/msgpack/v5"
)

type HttpClient struct {
	DefaultHeaders map[string]string
	HTTPClient     *http.Client
	RetrySchedule  []time.Duration
	BaseURL        string
	Debug          bool
}

func DefaultHttpClient(defaultBaseUrl string) HttpClient {
	tr := http.DefaultTransport.(*http.Transport).Clone()
	tr.ForceAttemptHTTP2 = true
	tr.TLSClientConfig = new(tls.Config)
	tr.TLSNextProto = make(map[string]func(authority string, c *tls.Conn) http.RoundTripper)

	return HttpClient{
		DefaultHeaders: map[string]string{},
		HTTPClient: &http.Client{
			Timeout:   60 * time.Second,
			Transport: tr,
		},
		RetrySchedule: []time.Duration{},
		BaseURL:       defaultBaseUrl,
		Debug:         false,
	}
}

func ExecuteRequest[ReqBody any, ResBody any](
	ctx context.Context,
	client *HttpClient,
	method string,
	path string,
	reqBody *ReqBody,
) (*ResBody, error) {
	urlStr := client.BaseURL + path

	var req *http.Request
	var err error
	if reqBody != nil {
		encodedBody, err := msgpack.Marshal(reqBody)
		if err != nil {
			return nil, newOtherError(err)
		}
		req, err = http.NewRequestWithContext(ctx, method, urlStr, bytes.NewBuffer(encodedBody))
		if err != nil {
			return nil, newOtherError(err)
		}
		req.Header.Set("content-type", "application/msgpack")
	} else {
		req, err = http.NewRequestWithContext(ctx, method, urlStr, &bytes.Buffer{})
		if err != nil {
			return nil, newOtherError(err)
		}
	}

	req.Header.Set("accept", "application/msgpack")
	req.Header.Set("diom-req-id", strconv.FormatUint(rand.Uint64(), 10))
	for hKey, hVal := range client.DefaultHeaders {
		req.Header.Add(hKey, hVal)
	}

	res, err := executeRequestWithRetries(client, req)

	if err != nil {
		return nil, err
	}
	if res.StatusCode == 204 {
		return nil, nil
	}
	defer res.Body.Close()

	body, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, newConnectionError(err)
	}

	if res.StatusCode >= 200 && res.StatusCode <= 299 {
		var ret ResBody
		err = msgpack.Unmarshal(body, &ret)
		if err != nil {
			return nil, newOtherError(err)
		}

		return &ret, nil
	}

	return nil, newResponseError(body)
}

func executeRequestWithRetries(client *HttpClient, request *http.Request) (*http.Response, error) {
	var bodyBytes []byte
	if request.Body != nil {
		var err error
		bodyBytes, err = io.ReadAll(request.Body)
		if err != nil {
			return nil, newConnectionError(err)
		}
		err = request.Body.Close()
		if err != nil {
			return nil, newConnectionError(err)
		}
	}

	if bodyBytes != nil {
		request.Body = io.NopCloser(bytes.NewReader(bodyBytes))
	}

	if client.Debug {
		log.Printf("URL: %s", request.URL)
		dump, err := httputil.DumpRequestOut(request, true)
		if err != nil {
			return nil, newOtherError(err)
		}
		log.Printf("\n%s\n", string(dump))
	}

	resp, err := client.HTTPClient.Do(request)
	for try := 0; try < len(client.RetrySchedule); try++ {
		if err == nil && resp.StatusCode < 500 {
			break
		}
		if bodyBytes != nil {
			request.Body = io.NopCloser(bytes.NewReader(bodyBytes))
		}
		request.Header.Set("diom-retry-count", strconv.Itoa(try+1))
		sleepTime := client.RetrySchedule[try]
		time.Sleep(sleepTime)
		resp, err = client.HTTPClient.Do(request)
	}

	if client.Debug {
		if resp != nil {
			dump, err := httputil.DumpResponse(resp, true)
			if err != nil {
				return resp, newOtherError(err)
			}
			log.Printf("\n%s\n", string(dump))
		}
	}

	if err != nil {
		return nil, newConnectionError(err)
	} else {
		return resp, nil
	}
}
