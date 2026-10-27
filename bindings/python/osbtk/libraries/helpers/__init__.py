"""The shared motion-graphics helpers (M38/M40): one ``helper`` op namespaced by
library (``keyframe.*``, ``scene3d.*``, ``command.*``, ``origin.*``, ``easing.*``,
``timeline.*``, ``color.*``, ``scene2d.*``).

The surface is code-split by namespace into this package; ``Helpers`` composes the
mixins so ``osb.helpers.call(...)`` and the namespaced methods all live on one
object.
"""

from __future__ import annotations

from osbtk.libraries.helpers._base import _Base
from osbtk.libraries.helpers._color import _Color
from osbtk.libraries.helpers._command import _Command
from osbtk.libraries.helpers._easing import _Easing
from osbtk.libraries.helpers._keyframe import _Keyframe
from osbtk.libraries.helpers._origin import _Origin
from osbtk.libraries.helpers._scene2d import _Scene2d
from osbtk.libraries.helpers._scene3d import _Scene3d
from osbtk.libraries.helpers._timeline import _Timeline


class Helpers(_Base, _Keyframe, _Origin, _Easing, _Command, _Timeline, _Color, _Scene2d, _Scene3d):
    """The shared motion-graphics helper surface."""


__all__ = ["Helpers"]
