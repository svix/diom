package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type AdminMetrics struct {
	client *diom_proto.HttpClient
}

func NewAdminMetrics(client *diom_proto.HttpClient) AdminMetrics {
	return AdminMetrics{client}
}

// Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver)
func (adminMetrics AdminMetrics) Get(
	ctx context.Context,
) (*diom_models.GetMetricsOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.GetMetricsOut](
		ctx,
		adminMetrics.client,
		"GET",
		"/api/v1.admin.metrics.get",
		nil,
	)
}
