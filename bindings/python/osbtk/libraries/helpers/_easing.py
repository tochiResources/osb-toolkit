"""The easing-curve manipulation helpers (M39)."""

from __future__ import annotations

from typing import List

from osbtk.libraries.helpers._base import call


class _Easing:
    """The easing-curve manipulation helpers."""

    @staticmethod
    def amplify_easing(easing: str, t: float, amount: float) -> float:
        """Amplifies an easing about linear by `amount` (>1 stronger)."""
        return call("easing.amplifyEasing", {"easing": easing, "t": t, "amount": amount})

    @staticmethod
    def sample_easing(easing: str, count: int = 8) -> List[float]:
        """Samples an easing at `count` points in [0, 1]."""
        return call("easing.sampleEasing", {"easing": easing, "count": count})
