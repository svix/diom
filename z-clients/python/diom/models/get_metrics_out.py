# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .metric_out import MetricOut


class GetMetricsOut(BaseModel):
    metrics: t.List[MetricOut]
