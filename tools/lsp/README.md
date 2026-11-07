# raptor-lsp

Language Server Protocol host for Raptor.

## Architecture

```text
Editor ──LSP──► raptor-lsp (Rust, tower-lsp)
                    │
                    │  subprocess + JSON on stdout
                    ▼
              lsp-backend (Raptor, tools/lsp/backend)
                    │
                    └── lex → parse → import → macro → semantic
                        → { ok, diagnostics, hovers, definitions }
```

- **`raptor-lsp`**: speaks LSP; owns document buffers; publishes diagnostics; answers hover / definition / completion.
- **`lsp-backend`**: self-hosted analysis binary; **libc-only** at runtime, same as other Raptor programs.

There is **no FFI** into the compiler: communication is a subprocess and one JSON object per analysis.

## Build the LSP host

```bash
cd tools/lsp
cargo build --release
# → target/release/raptor-lsp
```

## Build the backend

Using the bootstrapped compiler:

```bash
./bootstrap/raptor tools/lsp/backend/lsp-backend.rp -o ./build/lsp-backend
```

## Configuration

```bash
export RAPTOR_LSP_BACKEND=/absolute/path/to/build/lsp-backend
```

If unset, `raptor-lsp` tries `lsp-backend` on `PATH`, then `./build/lsp-backend`.

### VS Code

Extension settings (`plugins/vscode`):

```json
{
  "raptor.serverPath": "/path/to/tools/lsp/target/release/raptor-lsp",
  "raptor.useWsl": true
}
```

Set `RAPTOR_LSP_BACKEND` in the environment inherited by the server process.

## JSON payload (backend → host)

```json
{
  "ok": true,
  "diagnostics": [
    {
      "severity": "error",
      "message": "…",
      "phase": "semantic",
      "span": {
        "file": "…",
        "start_line": 1,
        "start_column": 1,
        "end_line": 1,
        "end_column": 5,
        "start_offset": 0,
        "end_offset": 4
      }
    }
  ],
  "definitions": [{ "use": { "…": "…" }, "def": { "…": "…" } }],
  "hovers": [{ "span": { "…": "…" }, "contents": "```raptor\n…" }]
}
```

## Capabilities

| LSP feature | Source |
| ----------- | ------ |
| Diagnostics | backend `diagnostics` |
| Hover | backend `hovers` |
| Go to definition | backend `definitions` |
| Completion | local (keywords, std names, identifiers in buffer) |
