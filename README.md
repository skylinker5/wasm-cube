# wasm-cube

Minimal **Rust → WebAssembly** + **TypeScript (Vite)** demo that draws a WebGL triangle on a `<canvas>`.

The frontend loads the WASM bundle from `wasm/pkg/wasm_cube` and calls `start("canvas")` (see `src/main.ts`).

## Prerequisites

- **Node.js** (includes `npm`)
- **Rust** toolchain (stable)
- **wasm-pack**

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

## Run locally (Windows / PowerShell)

Build the Rust WASM package:

```powershell
cd wasm
wasm-pack build --target web --out-dir pkg
cd ..
```

Install JS deps and start Vite:

```powershell
npm install
npm run dev
```

Then open the URL shown by Vite (usually `http://localhost:5173`).

## Production build

Build WASM first:

```powershell
cd wasm
wasm-pack build --target web --out-dir pkg
cd ..
```

Then build the site:

```powershell
npm run build
npm run preview
```

## Notes / troubleshooting

- **If `src/main.ts` fails to import `../wasm/pkg/wasm_cube`**: run the `wasm-pack build ...` step again to regenerate `wasm/pkg/`.
- **WebGL not available**: ensure your browser supports WebGL and hardware acceleration is enabled.

