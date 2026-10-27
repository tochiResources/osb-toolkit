"""The SDK's hand-written library facades, one module per library.

`osbtk.osb` composes these into the entry point; consumers use
``osb.curve.length(...)`` etc. rather than the generated clients.
"""

from __future__ import annotations

from osbtk.libraries.animations import Animations
from osbtk.libraries.audio import Audio
from osbtk.libraries.curve import Curve
from osbtk.libraries.diagnostics import Diagnostics
from osbtk.libraries.helpers import Helpers
from osbtk.libraries.image import Image
from osbtk.libraries.image_gen import ImageGen
from osbtk.libraries.media import Media
from osbtk.libraries.optimize import Optimize
from osbtk.libraries.storyboarding import Storyboarding
from osbtk.libraries.storyboarding3d import Storyboarding3d

__all__ = [
    "Animations",
    "Audio",
    "Curve",
    "Diagnostics",
    "Helpers",
    "Image",
    "ImageGen",
    "Media",
    "Optimize",
    "Storyboarding",
    "Storyboarding3d",
]
