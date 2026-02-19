package coyote

import (
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
