# this file is @generated

from ..internal.base_model import BaseModel

from .seek_position import SeekPosition
from .sink_config import SinkConfig


class SinkConfigureIn(BaseModel):
    namespace: str | None = None

    topic: str
    """The topic whose messages are forwarded to the sink. Created automatically if it does not
    exist."""

    consumer_group: str
    """The consumer group that identifies the sink and tracks its progress through the topic."""

    default_starting_position: SeekPosition | None = None
    """Where a freshly-created sink starts consuming the topic. Defaults to `earliest`."""

    max_in_flight: int | None = None
    """At most how many concurrent requests will be sent to the Sink."""

    config: SinkConfig
