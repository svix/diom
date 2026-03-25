# This file is @generated

from ..internal.api_common import ApiBase
from .admin_cluster import (
    AdminCluster,
    AdminClusterAsync,
)


class AdminAsync(ApiBase):
    @property
    def cluster(self) -> AdminClusterAsync:
        return AdminClusterAsync(self._client)


class Admin(ApiBase):
    @property
    def cluster(self) -> AdminCluster:
        return AdminCluster(self._client)
