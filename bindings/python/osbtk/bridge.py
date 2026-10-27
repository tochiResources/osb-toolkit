"""Bridge transport: routes the generated clients to the native ctypes bridge.

The SDK talks to the C ABI, not HTTP. This ``RESTClientObject`` replaces the
generated urllib3 transport: the operation is the last path segment (which is
exactly the native ``op``) and the request body is merged with it.
"""

from __future__ import annotations

import json
from typing import Any, Callable

from osbtk import rest

Bridge = Callable[[str], dict[str, Any]]


class _BridgeResponse:
    """The minimal urllib3 response shape ``RESTResponse`` reads."""

    def __init__(self, data: bytes, status: int, headers: dict[str, str]) -> None:
        self.data = data
        self.status = status
        self.reason = "OK"
        self.headers = headers


class BridgeRestClient(rest.RESTClientObject):
    """A ``RESTClientObject`` that calls the native JSON bridge instead of HTTP."""

    def __init__(self, bridge: Bridge) -> None:
        self._bridge = bridge

    def request(
        self,
        method: str,
        url: str,
        headers: dict[str, str] | None = None,
        body: Any = None,
        post_params: list[tuple[str, Any]] | None = None,
        _request_timeout: Any = None,
    ) -> rest.RESTResponse:
        del method, headers, post_params, _request_timeout
        op = url.rstrip("/").split("/")[-1]
        if isinstance(body, str):
            request: dict[str, Any] = json.loads(body) if body else {}
        elif isinstance(body, dict):
            request = dict(body)
        else:
            request = {}
        request["op"] = op
        result = self._bridge(json.dumps(request))
        status = 400 if isinstance(result, dict) and "error" in result else 200
        data = json.dumps(result).encode("utf-8")
        return rest.RESTResponse(_BridgeResponse(data, status, {"Content-Type": "application/json"}))
