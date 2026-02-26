# This file is @generated

from .common import ApiBase
from ..models import (
    CreateMsgTopicIn,
    CreateMsgTopicOut,
    GetMsgTopicIn,
    GetMsgTopicOut,
)


class MsgsTopicAsync(ApiBase):
    async def create(
        self,
        create_msg_topic_in: CreateMsgTopicIn,
    ) -> CreateMsgTopicOut:
        """Upserts a new message topic with the given name."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/topic/create",
            path_params={},
            json_body=create_msg_topic_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CreateMsgTopicOut.model_validate(response.json())

    async def get(
        self,
        get_msg_topic_in: GetMsgTopicIn,
    ) -> GetMsgTopicOut:
        """Get message topic with given name."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/topic/get",
            path_params={},
            json_body=get_msg_topic_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return GetMsgTopicOut.model_validate(response.json())


class MsgsTopic(ApiBase):
    def create(
        self,
        create_msg_topic_in: CreateMsgTopicIn,
    ) -> CreateMsgTopicOut:
        """Upserts a new message topic with the given name."""
        response = self._request_sync(
            method="post",
            path="/api/v1/msgs/topic/create",
            path_params={},
            json_body=create_msg_topic_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CreateMsgTopicOut.model_validate(response.json())

    def get(
        self,
        get_msg_topic_in: GetMsgTopicIn,
    ) -> GetMsgTopicOut:
        """Get message topic with given name."""
        response = self._request_sync(
            method="post",
            path="/api/v1/msgs/topic/get",
            path_params={},
            json_body=get_msg_topic_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return GetMsgTopicOut.model_validate(response.json())
