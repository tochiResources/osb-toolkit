# Third-party notices

osb-toolkit (osu! Storyboarding Toolkit) is distributed under the BSD-3-Clause license (see [`LICENSE`](./LICENSE)).
It bundles or depends on the following third-party components:

## OpenFX (bundled headers)

- **Component:** OpenFX image processing plug-in standard — headers vendored at
  `vendor/openfx/` (`include/ofxCore.h`, `ofxImageEffect.h`, `ofxParam.h`,
  `ofxProperty.h`, …).
- **Source:** https://github.com/AcademySoftwareFoundation/openfx
- **License:** BSD 3-Clause

```
BSD 3-Clause License

Copyright (c) 2025, OpenFX and contributors to the OpenFX project

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## Adobe After Effects / Premiere Pro SDK

The Adobe After Effects and Premiere Pro SDKs are **not** bundled in this
repository. They are installed separately by the user (`AfterEffectsSDK_26.5_win`,
or via the Premiere Pro SDK) and are used only at build time under Adobe's own
SDK license terms. Generated `.aex` plug-ins are built against the SDK the user
supplies.

## Rust dependencies

Rust crates used by the runtime and adapters (e.g. `thiserror`, `rustfft`,
`image`) are licensed under their own permissive licenses (MIT / Apache-2.0);
their license texts ship with the crates in the Cargo registry.
