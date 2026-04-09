# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    AdminAccessPolicyDeleteIn,
    AdminAccessPolicyDeleteOut,
    AdminAccessPolicyGetIn,
    AdminAccessPolicyListIn,
    AdminAccessPolicyOut,
    AdminAccessPolicyUpsertIn,
    AdminAccessPolicyUpsertOut,
    ListResponseAdminAccessPolicyOut,
)


class AdminAuthPolicyAsync(ApiBase):
    async def upsert(
        self,
        admin_access_policy_upsert_in: AdminAccessPolicyUpsertIn,
    ) -> AdminAccessPolicyUpsertOut:
        """Create or update an access policy"""
        body = admin_access_policy_upsert_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.upsert",
            body=body,
            response_type=AdminAccessPolicyUpsertOut,
        )

    async def delete(
        self,
        admin_access_policy_delete_in: AdminAccessPolicyDeleteIn,
    ) -> AdminAccessPolicyDeleteOut:
        """Delete an access policy"""
        body = admin_access_policy_delete_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.delete",
            body=body,
            response_type=AdminAccessPolicyDeleteOut,
        )

    async def get(
        self,
        admin_access_policy_get_in: AdminAccessPolicyGetIn,
    ) -> AdminAccessPolicyOut:
        """Get an access policy by ID"""
        body = admin_access_policy_get_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.get",
            body=body,
            response_type=AdminAccessPolicyOut,
        )

    async def list(
        self,
        admin_access_policy_list_in: AdminAccessPolicyListIn = AdminAccessPolicyListIn(),
    ) -> ListResponseAdminAccessPolicyOut:
        """List all access policies"""
        body = admin_access_policy_list_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-policy.list",
            body=body,
            response_type=ListResponseAdminAccessPolicyOut,
        )


class AdminAuthPolicy(ApiBase):
    def upsert(
        self,
        admin_access_policy_upsert_in: AdminAccessPolicyUpsertIn,
    ) -> AdminAccessPolicyUpsertOut:
        """Create or update an access policy"""
        body = admin_access_policy_upsert_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.upsert",
            body=body,
            response_type=AdminAccessPolicyUpsertOut,
        )

    def delete(
        self,
        admin_access_policy_delete_in: AdminAccessPolicyDeleteIn,
    ) -> AdminAccessPolicyDeleteOut:
        """Delete an access policy"""
        body = admin_access_policy_delete_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.delete",
            body=body,
            response_type=AdminAccessPolicyDeleteOut,
        )

    def get(
        self,
        admin_access_policy_get_in: AdminAccessPolicyGetIn,
    ) -> AdminAccessPolicyOut:
        """Get an access policy by ID"""
        body = admin_access_policy_get_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.get",
            body=body,
            response_type=AdminAccessPolicyOut,
        )

    def list(
        self,
        admin_access_policy_list_in: AdminAccessPolicyListIn = AdminAccessPolicyListIn(),
    ) -> ListResponseAdminAccessPolicyOut:
        """List all access policies"""
        body = admin_access_policy_list_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.admin.auth-policy.list",
            body=body,
            response_type=ListResponseAdminAccessPolicyOut,
        )
