# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgTopicConfigureIn,
    MsgTopicConfigureOut,
)

from ..models.msg_topic_configure_in import _MsgTopicConfigureIn


class MsgsTopicAsync(ApiBase):
    async def configure(
        self,
        topic: str,
        partitions: int,
        msg_topic_configure_in: MsgTopicConfigureIn,
    ) -> MsgTopicConfigureOut:
        """Configures the number of partitions for a topic.

        Partition count can only be increased, never decreased. The default for a new topic is 1."""
        body = _MsgTopicConfigureIn(
            topic=topic,
            partitions=partitions,
        ).model_dump(exclude_unset=True, by_alias=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/topic/configure",
            body=body,
            response_type=MsgTopicConfigureOut,
        )


class MsgsTopic(ApiBase):
    def configure(
        self,
        topic: str,
        partitions: int,
        msg_topic_configure_in: MsgTopicConfigureIn,
    ) -> MsgTopicConfigureOut:
        """Configures the number of partitions for a topic.

        Partition count can only be increased, never decreased. The default for a new topic is 1."""
        body = _MsgTopicConfigureIn(
            topic=topic,
            partitions=partitions,
        ).model_dump(exclude_unset=True, by_alias=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/topic/configure",
            body=body,
            response_type=MsgTopicConfigureOut,
        )
