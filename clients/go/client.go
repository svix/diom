package diom

import (
	"net/http"
	"net/url"
	"time"

	diom_proto "github.com/svix/diom/clients/go/internal/proto"
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
