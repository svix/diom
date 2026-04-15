# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    MsgNamespaceConfigureIn,
    MsgNamespaceConfigureOut,
    MsgNamespaceGetIn,
    MsgNamespaceGetOut,
)

from ..models.msg_namespace_configure_in import _MsgNamespaceConfigureIn
from ..models.msg_namespace_get_in import _MsgNamespaceGetIn


class MsgsNamespaceAsync(ApiBase):
    async def configure(
        self,
        name: str,
        msg_namespace_configure_in: MsgNamespaceConfigureIn = MsgNamespaceConfigureIn(),
    ) -> MsgNamespaceConfigureOut:
        """Configures a msgs namespace with the given name."""
        body = _MsgNamespaceConfigureIn(
            name=name,
            retention=msg_namespace_configure_in.retention,
        ).model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.namespace.configure",
            body=body,
        )
        return parse_response(response, MsgNamespaceConfigureOut)

    async def get(
        self,
        name: str,
        msg_namespace_get_in: MsgNamespaceGetIn = MsgNamespaceGetIn(),
    ) -> MsgNamespaceGetOut:
        """Gets a msgs namespace by name."""
        body = _MsgNamespaceGetIn(
            name=name,
        ).model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.namespace.get",
            body=body,
        )
        return parse_response(response, MsgNamespaceGetOut)


class MsgsNamespace(ApiBase):
    def configure(
        self,
        name: str,
        msg_namespace_configure_in: MsgNamespaceConfigureIn = MsgNamespaceConfigureIn(),
    ) -> MsgNamespaceConfigureOut:
        """Configures a msgs namespace with the given name."""
        body = _MsgNamespaceConfigureIn(
            name=name,
            retention=msg_namespace_configure_in.retention,
        ).model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.msgs.namespace.configure",
            body=body,
        )
        return parse_response(response, MsgNamespaceConfigureOut)

    def get(
        self,
        name: str,
        msg_namespace_get_in: MsgNamespaceGetIn = MsgNamespaceGetIn(),
    ) -> MsgNamespaceGetOut:
        """Gets a msgs namespace by name."""
        body = _MsgNamespaceGetIn(
            name=name,
        ).model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.msgs.namespace.get",
            body=body,
        )
        return parse_response(response, MsgNamespaceGetOut)
