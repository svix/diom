# this file is @generated

from ..internal.base_model import BaseModel


class MsgTopicPartitionDescribeOut(BaseModel):
    partition_id: int

    high_water_mark: int
    """The next offset to be committed to this partition"""
