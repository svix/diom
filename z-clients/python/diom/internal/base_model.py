import pydantic


class BaseModel(pydantic.BaseModel):
    model_config = pydantic.ConfigDict(serialize_by_alias=True)
