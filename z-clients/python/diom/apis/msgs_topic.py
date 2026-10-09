# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ListResponseMsgTopicListOut,
    MsgTopicConfigureIn,
    MsgTopicConfigureOut,
    MsgTopicDescribeIn,
    MsgTopicDescribeOut,
    MsgTopicListIn,
)

from ..models.msg_topic_configure_in import _MsgTopicConfigureIn
from ..models.msg_topic_describe_in import _MsgTopicDescribeIn


class MsgsTopicAsync(ApiBase):
    async def configure(
        self,
        topic: str,
        msg_topic_configure_in: MsgTopicConfigureIn,
    ) -> MsgTopicConfigureOut:
        """Configures the number of partitions for a topic.

        Partition count can only be increased, never decreased. The default for a new topic is 1."""
        body = _MsgTopicConfigureIn(
            namespace=msg_topic_configure_in.namespace,
            topic=topic,
            partitions=msg_topic_configure_in.partitions,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.topic.configure",
            body=body,
            response_type=MsgTopicConfigureOut,
        )

    async def list(
        self,
        msg_topic_list_in: MsgTopicListIn = MsgTopicListIn(),
    ) -> ListResponseMsgTopicListOut:
        """List available topics in the given namespace"""
        body = msg_topic_list_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.topic.list",
            body=body,
            response_type=ListResponseMsgTopicListOut,
        )

    async def describe(
        self,
        topic: str,
        msg_topic_describe_in: MsgTopicDescribeIn = MsgTopicDescribeIn(),
    ) -> MsgTopicDescribeOut:
        """Show information about the given topic"""
        body = _MsgTopicDescribeIn(
            namespace=msg_topic_describe_in.namespace,
            topic=topic,
            consistency=msg_topic_describe_in.consistency,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.topic.describe",
            body=body,
            response_type=MsgTopicDescribeOut,
        )


class MsgsTopic(ApiBase):
    def configure(
        self,
        topic: str,
        msg_topic_configure_in: MsgTopicConfigureIn,
    ) -> MsgTopicConfigureOut:
        """Configures the number of partitions for a topic.

        Partition count can only be increased, never decreased. The default for a new topic is 1."""
        body = _MsgTopicConfigureIn(
            namespace=msg_topic_configure_in.namespace,
            topic=topic,
            partitions=msg_topic_configure_in.partitions,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.topic.configure",
            body=body,
            response_type=MsgTopicConfigureOut,
        )

    def list(
        self,
        msg_topic_list_in: MsgTopicListIn = MsgTopicListIn(),
    ) -> ListResponseMsgTopicListOut:
        """List available topics in the given namespace"""
        body = msg_topic_list_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.topic.list",
            body=body,
            response_type=ListResponseMsgTopicListOut,
        )

    def describe(
        self,
        topic: str,
        msg_topic_describe_in: MsgTopicDescribeIn = MsgTopicDescribeIn(),
    ) -> MsgTopicDescribeOut:
        """Show information about the given topic"""
        body = _MsgTopicDescribeIn(
            namespace=msg_topic_describe_in.namespace,
            topic=topic,
            consistency=msg_topic_describe_in.consistency,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.topic.describe",
            body=body,
            response_type=MsgTopicDescribeOut,
        )
