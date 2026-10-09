# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_topic_partition_describe_out import MsgTopicPartitionDescribeOut


class MsgTopicDescribeOut(BaseModel):
    id: str
    """The unique internal ID of this topic

    This can useful for debugging"""

    name: str

    partitions: t.List[MsgTopicPartitionDescribeOut]
