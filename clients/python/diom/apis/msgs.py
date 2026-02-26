# This file is @generated

from .common import ApiBase
from .msgs_topic import (
    MsgsTopic,
    MsgsTopicAsync,
)


class MsgsAsync(ApiBase):
    @property
    def topic(self) -> MsgsTopicAsync:
        return MsgsTopicAsync(self._client)


class Msgs(ApiBase):
    @property
    def topic(self) -> MsgsTopic:
        return MsgsTopic(self._client)
