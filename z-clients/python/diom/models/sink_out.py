# this file is @generated

from ..internal.base_model import BaseModel

from .seek_position import SeekPosition
from .sink_config import SinkConfig


class SinkOut(BaseModel):
    topic: str

    consumer_group: str

    default_starting_position: SeekPosition | None = None
    """Where a freshly-created sink starts consuming the topic. Defaults to `earliest`."""

    max_in_flight: int | None = None
    """At most how many concurrent requests will be sent to the Sink."""

    config: SinkConfig
