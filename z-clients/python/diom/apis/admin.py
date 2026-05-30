# This file is @generated

from ..internal.api_common import ApiBase
from .admin_auth_policy import (
    AdminAuthPolicy,
    AdminAuthPolicyAsync,
)
from .admin_auth_role import (
    AdminAuthRole,
    AdminAuthRoleAsync,
)
from .admin_auth_token import (
    AdminAuthToken,
    AdminAuthTokenAsync,
)
from .admin_metrics import (
    AdminMetrics,
    AdminMetricsAsync,
)


class AdminAsync(ApiBase):
    @property
    def auth_policy(self) -> AdminAuthPolicyAsync:
        return AdminAuthPolicyAsync(self._client)

    @property
    def auth_role(self) -> AdminAuthRoleAsync:
        return AdminAuthRoleAsync(self._client)

    @property
    def auth_token(self) -> AdminAuthTokenAsync:
        return AdminAuthTokenAsync(self._client)

    @property
    def metrics(self) -> AdminMetricsAsync:
        return AdminMetricsAsync(self._client)


class Admin(ApiBase):
    @property
    def auth_policy(self) -> AdminAuthPolicy:
        return AdminAuthPolicy(self._client)

    @property
    def auth_role(self) -> AdminAuthRole:
        return AdminAuthRole(self._client)

    @property
    def auth_token(self) -> AdminAuthToken:
        return AdminAuthToken(self._client)

    @property
    def metrics(self) -> AdminMetrics:
        return AdminMetrics(self._client)
