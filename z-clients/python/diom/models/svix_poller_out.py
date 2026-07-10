# this file is @generated

from ..internal.base_model import BaseModel


class SvixPollerOut(BaseModel):
    topic: str

    poller_id: str

    token: str
    """The autoconfig token, obfuscated (e.g. `auto_v1_eyJh...fQ==`)."""
