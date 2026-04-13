from datetime import datetime, timedelta
from typing import Annotated
from pydantic import PlainSerializer, PlainValidator


def _validate_duration_ms(v: object) -> timedelta:
    if isinstance(v, timedelta):
        # used by constructor calls
        return v
    if isinstance(v, int):
        # used by model_validate
        return timedelta(milliseconds=v)
    else:
        raise ValueError(f"Expected integer or timedelta, got {type(v)}")


def _serialize_duration_ms(td: timedelta) -> int:
    return td.seconds * 1000 + int(round(td.microseconds / 1000))


TimeDeltaMs = Annotated[
    timedelta,
    PlainSerializer(_serialize_duration_ms, return_type=int),
    PlainValidator(_validate_duration_ms, json_schema_input_type=int),
]


def _validate_unix_timestamp_ms(v: object) -> datetime:
    if isinstance(v, datetime):
        # used by constructor calls
        return v
    if isinstance(v, int):
        # used by model_validate
        return datetime.fromtimestamp(v / 1000)
    else:
        raise ValueError(f"Expected integer or datetime, got {type(v)}")


def _serialize_unix_timestamp_ms(dt: datetime) -> int:
    return int(round(dt.timestamp() * 1000))


UnixTimestampMs = Annotated[
    datetime,
    PlainSerializer(_serialize_unix_timestamp_ms, return_type=int),
    PlainValidator(_validate_unix_timestamp_ms, json_schema_input_type=int),
]
