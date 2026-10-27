"""The shared stateless bridge client for the SDK libraries (M33).

Every library facade in :mod:`osbtk.libraries` drives the same stateless
:class:`~osbtk.api_client.ApiClient`, wired to the native engine through
:class:`~osbtk.bridge.BridgeRestClient`.
"""

from __future__ import annotations

from osbtk._native import exec
from osbtk.api_client import ApiClient
from osbtk.bridge import BridgeRestClient


def _stateless_client() -> ApiClient:
    client = ApiClient()
    client.rest_client = BridgeRestClient(exec)
    return client


STATELESS = _stateless_client()
