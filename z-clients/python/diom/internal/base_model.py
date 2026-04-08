import pydantic


class BaseModel(pydantic.BaseModel):
    model_config = pydantic.ConfigDict(
        serialize_by_alias=True,
        # Use declared field names for constructor calls, not aliases.
        validate_by_name=True,
        validate_by_alias=False,
    )
