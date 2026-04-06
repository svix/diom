from datetime import timedelta
from typing import Annotated
from pydantic import PlainSerializer, PlainValidator


def validate(v: object) -> timedelta:
    if isinstance(v, timedelta):
        # used by constructor calls
        return v
    if isinstance(v, int):
        # used by model_validate
        return timedelta(milliseconds=v)
    else:
        raise ValueError(f"Expected integer or timedelta, got {type(v)}")


def serialize(td: timedelta) -> int:
    return td.seconds * 1000 + int(round(td.microseconds / 1000))


TimeDeltaMs = Annotated[
    timedelta,
    PlainSerializer(serialize, return_type=int),
    PlainValidator(validate, json_schema_input_type=int),
]
