# zed-sapvc

Zed editor extension for the **SAP Variant Configuration (LO-VC / AVC) dependency language**.

Adds syntax highlighting and language-server support for VC dependency source files
(`PRO_*`, `CONS_*`).

## Requirements

- [Zed](https://zed.dev)
- The [`sapvc-lsp`](https://github.com/zpage/sapvc-lsp) binary, built with
  `cargo build --release`
- The [`tree-sitter-sapvc`](https://github.com/zpage/tree-sitter-sapvc) grammar

## Structure

```
extension.toml                 grammar declaration (rev pinned to tree-sitter-sapvc)
languages/sapvc/config.toml    language name, grammar, comment tokens, file patterns
languages/sapvc/highlights.scm tree-sitter highlight queries
src/lib.rs                     LSP spawn shell (wasm32-wasip1)
```

## Install as a development extension

1. Build the grammar:

   ```bash
   cd /path/to/tree-sitter-sapvc
   npm install
   npm run generate
   ```

2. Put this directory under your Zed dev extensions folder:

   ```
   <zed-config>/extensions/dev/sapvc/
   ```

3. Restart Zed and open a `.sapvc` file, `PRO_*.txt`, or `CONS_*.txt`.
   The status bar shows `SAP VC` when detection works.

If detection fails, add explicit file patterns in your Zed settings:

```json
"file_types": { "SAP VC": ["**/PRO_*.txt", "**/CONS_*.txt"] }
```

## Configuration

The extension spawns the language server through two environment variables:

| Variable | Purpose |
|---|---|
| `SAPVC_LSP_BIN` | Path to the `sapvc-lsp` binary. Defaults to `sapvc-lsp` on `PATH`. |
| `SAPVC_MATERIAL` | Optional path to a material data package JSON. Enables semantic checks. |

Without `SAPVC_MATERIAL` the server still provides syntax diagnostics and highlighting.

## What you get

- `$SELF` / `$ROOT` / `$PARENT` highlighted as built-in variables
- Operators: `?=`, `IN`, `SPECIFIED`, `AND` / `OR` / `NOT`, `is invisible`
- Function colors for `$DEL_DEFAULT`, `TABLE`, `PFUNCTION`
- Section keywords: `OBJECTS:`, `CONDITION:`, `RESTRICTIONS:`, `INFERENCES:`
- `*` comments styled as comments
- Real-time diagnostics: unknown characteristic names, unknown variant tables,
  missing colons, unmatched blocks

## Notes

- The highlight query uses the node name `comment_statement`, not `comment`.
  Zed refuses to load a language whose query names a node that the grammar
  does not define.
- `settings.json` `file_types` globs do not apply in dev extension mode. Use the
  `path_suffixes` / `path_prefixes` rules in `languages/sapvc/config.toml`.
