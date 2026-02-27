# this file is @generated
import typing as t

from .common import BaseModel

from .publish_out_msg import PublishOutMsg


class PublishOut(BaseModel):
    msgs: t.List[PublishOutMsg]
