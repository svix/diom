# this file is @generated
import typing as t

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs

from .metric_type import MetricType


class MetricOut(BaseModel):
    label: str
    """Label for this series"""

    description: str
    """Human-readable description of this series"""

    attributes: t.Dict[str, str]
    """Key/Value pairs attached to this sequence"""

    value: float
    """Most recent data point for this series

    All points (u64, i64, and f64) are squished into an f64, be careful
    of inexactness for values above 2**53."""

    metric_type: MetricType
    """Type of this metric

    Histograms are not currently exported through this API, and can
    only be accessed through OTLP."""

    timestamp: UnixTimestampMs
    """Timestamp this metric was collected"""

    unit: str | None = None
    """Optional unit, following UCUM unit conventions if possible

    See https://ucum.org/ for details"""
