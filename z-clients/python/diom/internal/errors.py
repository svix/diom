import typing as t
from dataclasses import dataclass

from .base_model import BaseModel


@dataclass
class DiomError(Exception):
    operation_id: str


@dataclass
class ConnError(DiomError):
    pass


@dataclass
class InvalidInputError(DiomError):
    code: str
    detail: str
    location: str | None


@dataclass
class OperationError(DiomError):
    code: str
    detail: str
    location: str | None


@dataclass
class ServerError(DiomError):
    code: str
    detail: str
    location: str | None


@dataclass
class OtherError(DiomError):
    pass


class ErrorBody(BaseModel):
    type: str
    code: str
    detail: str
    location: str | None


def raise_diom_error(body: ErrorBody, op_id) -> t.NoReturn:
    match body.type:
        case "invalid-input":
            raise InvalidInputError(op_id, body.code, body.detail, body.location)
        case "operation-error":
            raise OperationError(op_id, body.code, body.detail, body.location)
        case "server-error":
            raise ServerError(op_id, body.code, body.detail, body.location)
        case _:
            raise OtherError(op_id) from ValueError(f"invalid error type `{body.type}`")
