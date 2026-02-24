package coyote

import (
	"fmt"
	"net/http"
	"net/url"
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type CoyoteOptions struct {
	ServerUrl     *url.URL
	HTTPClient    *http.Client
	RetrySchedule *[]time.Duration
	Debug         bool
}

type Coyote struct {
	inner coyote_proto.HttpClient
}

func New(token string, options *CoyoteOptions) (*Coyote, error) {
	httpClient := coyote_proto.DefaultHttpClient("http://localhost:8050")

	if options != nil {
		if options.ServerUrl != nil {
			httpClient.BaseURL = options.ServerUrl.String()
		}

		if options.RetrySchedule != nil {
			if len(*options.RetrySchedule) > 5 {
				return nil, fmt.Errorf("number of retries must not exceed 5")
			}
			httpClient.RetrySchedule = *options.RetrySchedule
		}

		if options.HTTPClient != nil {
			httpClient.HTTPClient = options.HTTPClient
		}

		httpClient.Debug = options.Debug
	}

	httpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	httpClient.DefaultHeaders["User-Agent"] = "coyote-sdks/0.0.1/go"

	client := Coyote{httpClient}
	return &client, nil
}
