package diom

import (
	"fmt"
	"net/http"
	"net/url"
	"time"

	diom_proto "diom.svix.com/go/diom/internal/proto"
)

type DiomOptions struct {
	ServerUrl     *url.URL
	HTTPClient    *http.Client
	RetrySchedule *[]time.Duration
	Debug         bool
}

type Diom struct {
	inner diom_proto.HttpClient
}

func New(token string, options *DiomOptions) (*Diom, error) {
	httpClient := diom_proto.DefaultHttpClient("http://localhost:8624")

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
	httpClient.DefaultHeaders["User-Agent"] = "diom-sdks/0.0.1/go"

	client := Diom{httpClient}
	return &client, nil
}
