# Web UI

The pure web version lives in `apps/frontend` and is published as
`@tochii/osb-toolkit-web`. It is built with **Vite 8**, **React 19** and
**CSS Modules**.

```bash
yarn dev:web      # dev server on http://localhost:5173
yarn w:web build  # production build to apps/frontend/dist
```

## What it does

1. **Drop a `.cs` file** (or load the bundled sample).
2. **Pick a target** — the card shows the host, technology and artifact.
3. **Convert** — the conversion runs entirely in the browser via
   `@tochii/osb-toolkit-core`.
4. **Inspect the output** — a file list, diagnostics and a code preview, with
   per-file download and a "download all as JSON" export.

## Using it from code

```ts
import { convertInBrowser } from '@tochii/osb-toolkit-web';

const result = convertInBrowser({
    target: 'after-effects',
    source,
    fileName: 'Background.cs',
});
```

The package re-exports the `App` component and the conversion helpers so the
desktop app can reuse the exact same UI.
