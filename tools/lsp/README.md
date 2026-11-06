# raptor-lsp

Language Server Protocol server for Raptor, built with [`tower-lsp`](https://crates.io/crates/tower-lsp).

Semantic analysis, hovers, definitions and diagnostics come from the **self-hosted**
`lsp-backend` (`tools/lsp-backend.rp`), not from the historical Rust checker in `src/`.

## Build

```bash
cd tools/lsp
cargo build --release
# binary: target/release/raptor-lsp
```

## Backend binary

Build the analysis helper once (self-hosted compiler):

```bash
# from the repository root, after bootstrap
./bootstrap/raptor tools/lsp-backend.rp -o ./build/lsp-backend
# or, if your driver uses --run / link scripts, produce an executable named lsp-backend
```

Point the server at it:

```bash
export RAPTOR_LSP_BACKEND=/absolute/path/to/build/lsp-backend
```

If `RAPTOR_LSP_BACKEND` is unset, the server tries, in order:

1. `lsp-backend` on `PATH`
2. `./build/lsp-backend` (relative to the server’s working directory)

## Protocol between Rust and Raptor

On each `textDocument/didOpen` / `didChange` / `didSave`:

1. The server writes the buffer to a temporary `.rp` file.
2. It runs: `lsp-backend <temp.rp>` and reads **stdout**.
3. The backend prints one JSON object (see `tools/lsp-backend.rp` header).
4. Temp-file paths inside spans are rewritten to the document’s real path.

There is **no FFI** — only a subprocess and JSON.

## Capabilities

| LSP feature              | Source                                      |
| ------------------------ | ------------------------------------------- |
| Diagnostics              | backend `diagnostics[]`                     |
| Hover                    | backend `hovers[]`                          |
| Go to definition         | backend `definitions[]`                     |
| Completion (keywords / std / identifiers) | local, no backend round-trip |

## VS Code / client

Point the client at `raptor-lsp` and set `RAPTOR_LSP_BACKEND` in the environment
inherited by the server process.
