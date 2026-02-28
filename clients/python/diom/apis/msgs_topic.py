# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    TopicConfigureIn,
    TopicConfigureOut,
)


class MsgsTopicAsync(ApiBase):
    async def configure(
        self,
        topic_configure_in: TopicConfigureIn,
    ) -> TopicConfigureOut:
        """Configures the number of partitions for a topic.

        Partition count can only be increased, never decreased. The default for a new topic is 1."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/topic/configure",
            body=topic_configure_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=TopicConfigureOut,
        )


class MsgsTopic(ApiBase):
    def configure(
        self,
        topic_configure_in: TopicConfigureIn,
    ) -> TopicConfigureOut:
        """Configures the number of partitions for a topic.

        Partition count can only be increased, never decreased. The default for a new topic is 1."""
        return self._request_sync(
            method="post",
            path="/api/v1/msgs/topic/configure",
            body=topic_configure_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=TopicConfigureOut,
        )
