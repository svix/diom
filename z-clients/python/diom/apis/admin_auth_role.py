# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    AdminRoleConfigureIn,
    AdminRoleConfigureOut,
    AdminRoleDeleteIn,
    AdminRoleDeleteOut,
    AdminRoleGetIn,
    AdminRoleListIn,
    AdminRoleOut,
    ListResponseAdminRoleOut,
)


class AdminAuthRoleAsync(ApiBase):
    async def configure(
        self,
        admin_role_configure_in: AdminRoleConfigureIn,
    ) -> AdminRoleConfigureOut:
        """Create or update a role"""
        body = admin_role_configure_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-role.configure",
            body=body,
        )
        return parse_response(response, AdminRoleConfigureOut)

    async def delete(
        self,
        admin_role_delete_in: AdminRoleDeleteIn,
    ) -> AdminRoleDeleteOut:
        """Delete a role"""
        body = admin_role_delete_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-role.delete",
            body=body,
        )
        return parse_response(response, AdminRoleDeleteOut)

    async def get(
        self,
        admin_role_get_in: AdminRoleGetIn,
    ) -> AdminRoleOut:
        """Get a role by ID"""
        body = admin_role_get_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-role.get",
            body=body,
        )
        return parse_response(response, AdminRoleOut)

    async def list(
        self,
        admin_role_list_in: AdminRoleListIn = AdminRoleListIn(),
    ) -> ListResponseAdminRoleOut:
        """List all roles"""
        body = admin_role_list_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.auth-role.list",
            body=body,
        )
        return parse_response(response, ListResponseAdminRoleOut)


class AdminAuthRole(ApiBase):
    def configure(
        self,
        admin_role_configure_in: AdminRoleConfigureIn,
    ) -> AdminRoleConfigureOut:
        """Create or update a role"""
        body = admin_role_configure_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-role.configure",
            body=body,
        )
        return parse_response(response, AdminRoleConfigureOut)

    def delete(
        self,
        admin_role_delete_in: AdminRoleDeleteIn,
    ) -> AdminRoleDeleteOut:
        """Delete a role"""
        body = admin_role_delete_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-role.delete",
            body=body,
        )
        return parse_response(response, AdminRoleDeleteOut)

    def get(
        self,
        admin_role_get_in: AdminRoleGetIn,
    ) -> AdminRoleOut:
        """Get a role by ID"""
        body = admin_role_get_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-role.get",
            body=body,
        )
        return parse_response(response, AdminRoleOut)

    def list(
        self,
        admin_role_list_in: AdminRoleListIn = AdminRoleListIn(),
    ) -> ListResponseAdminRoleOut:
        """List all roles"""
        body = admin_role_list_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.auth-role.list",
            body=body,
        )
        return parse_response(response, ListResponseAdminRoleOut)
