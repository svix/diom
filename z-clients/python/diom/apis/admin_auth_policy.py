# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    AdminAccessPolicyConfigureIn,
    AdminAccessPolicyConfigureOut,
    AdminAccessPolicyDeleteIn,
    AdminAccessPolicyDeleteOut,
    AdminAccessPolicyGetIn,
    AdminAccessPolicyListIn,
    AdminAccessPolicyOut,
    ListResponseAdminAccessPolicyOut,
)


class AdminAuthPolicyAsync(ApiBase):
    async def configure(
        self,
        admin_access_policy_configure_in: AdminAccessPolicyConfigureIn,
    ) -> AdminAccessPolicyConfigureOut:
        """Create or update an access policy"""
        body = admin_access_policy_configure_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.configure",
            body=body,
        )
        return parse_response(response, AdminAccessPolicyConfigureOut)

    async def delete(
        self,
        admin_access_policy_delete_in: AdminAccessPolicyDeleteIn,
    ) -> AdminAccessPolicyDeleteOut:
        """Delete an access policy"""
        body = admin_access_policy_delete_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.delete",
            body=body,
        )
        return parse_response(response, AdminAccessPolicyDeleteOut)

    async def get(
        self,
        admin_access_policy_get_in: AdminAccessPolicyGetIn,
    ) -> AdminAccessPolicyOut:
        """Get an access policy by ID"""
        body = admin_access_policy_get_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.get",
            body=body,
        )
        return parse_response(response, AdminAccessPolicyOut)

    async def list(
        self,
        admin_access_policy_list_in: AdminAccessPolicyListIn = AdminAccessPolicyListIn(),
    ) -> ListResponseAdminAccessPolicyOut:
        """List all access policies"""
        body = admin_access_policy_list_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.list",
            body=body,
        )
        return parse_response(response, ListResponseAdminAccessPolicyOut)


class AdminAuthPolicy(ApiBase):
    def configure(
        self,
        admin_access_policy_configure_in: AdminAccessPolicyConfigureIn,
    ) -> AdminAccessPolicyConfigureOut:
        """Create or update an access policy"""
        body = admin_access_policy_configure_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.configure",
            body=body,
        )
        return parse_response(response, AdminAccessPolicyConfigureOut)

    def delete(
        self,
        admin_access_policy_delete_in: AdminAccessPolicyDeleteIn,
    ) -> AdminAccessPolicyDeleteOut:
        """Delete an access policy"""
        body = admin_access_policy_delete_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.delete",
            body=body,
        )
        return parse_response(response, AdminAccessPolicyDeleteOut)

    def get(
        self,
        admin_access_policy_get_in: AdminAccessPolicyGetIn,
    ) -> AdminAccessPolicyOut:
        """Get an access policy by ID"""
        body = admin_access_policy_get_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.get",
            body=body,
        )
        return parse_response(response, AdminAccessPolicyOut)

    def list(
        self,
        admin_access_policy_list_in: AdminAccessPolicyListIn = AdminAccessPolicyListIn(),
    ) -> ListResponseAdminAccessPolicyOut:
        """List all access policies"""
        body = admin_access_policy_list_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.list",
            body=body,
        )
        return parse_response(response, ListResponseAdminAccessPolicyOut)
