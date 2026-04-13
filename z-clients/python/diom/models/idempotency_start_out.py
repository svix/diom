# this file is @generated
import typing as t
from typing_extensions import Self
from pydantic import ModelWrapValidatorHandler, model_validator

from ..internal.base_model import BaseModel


from .idempotency_completed import IdempotencyCompleted


class IdempotencyStartOut(BaseModel):
    status: t.Literal["started", "locked", "completed"]
    data: dict[str, t.Any] | IdempotencyCompleted

    @model_validator(mode="wrap")
    @classmethod
    def validate_model(
        cls, data: t.Any, handler: ModelWrapValidatorHandler[Self]
    ) -> Self:
        if "data" not in data:
            data["data"] = {}
        output = handler(data)
        if output.status == "started":
            output.data = data.get("data", {})
        elif output.status == "locked":
            output.data = data.get("data", {})
        elif output.status == "completed":
            output.data = IdempotencyCompleted.model_validate(data.get("data", {}))
        else:
            raise ValueError(f"Unexpected type `{output.status}`")
        return output
