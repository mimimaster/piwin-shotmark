# Icon source

`icon-source.png` is the generated 1024 px master; `src-tauri/icons/` is derived
from it with `npx tauri icon design/icon-source.png`.

The master was produced with an image model and then knocked out to a
transparent margin by `knockout.cjs` (the white backdrop is flooded away from
the edges only, so the white brackets inside the plate survive). Re-running the
knockout needs `npm i --no-save sharp`:

```bash
node design/knockout.cjs <generated-master.png> design/icon-source.png
npx tauri icon design/icon-source.png
```
