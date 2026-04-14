# this file is @generated
from enum import Enum


class ServerState(str, Enum):
    LEADER = "leader"
    FOLLOWER = "follower"
    LEARNER = "learner"
    CANDIDATE = "candidate"
    SHUTDOWN = "shutdown"
    UNKNOWN = "unknown"

    def __str__(self) -> str:
        return str(self.value)
