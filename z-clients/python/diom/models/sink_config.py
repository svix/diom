# this file is @generated
import typing as t
from typing_extensions import Self
from pydantic import ModelWrapValidatorHandler, model_validator

from ..internal.base_model import BaseModel


from .http_sink_config import HttpSinkConfig


class SinkConfig(BaseModel):
    type: t.Literal["http"]
    data: HttpSinkConfig

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if "data" not in data:
            data["data"] = {}
        output = handler(data)
        if output.type == "http":
            output.data = HttpSinkConfig.model_validate(data.get("data", {}))
        else:
            raise ValueError(f"Unexpected type `{output.type}`")
        return output
