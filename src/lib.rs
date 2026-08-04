//! Zed extension shell for SAP VC/AVC dependency files.
//!
//! The heavy lifting lives in the `sapvc-lsp` binary (tower-lsp + tree-sitter).
//! This shell only tells Zed how to spawn it, per the official extension API.
//!
//! Build: cargo build --release --target wasm32-wasip2
//! Output: target/wasm32-wasip2/release/zed-sapvc.wasm -> extension.wasm

use zed_extension_api as zed;

const SAPVC_LSP: &str =
    "C:/Users/10586006/01Project/05ProcessOptimize/sapvc-tools/sapvc-lsp/target/debug/sapvc-lsp.exe";

struct SapvcExtension;

impl zed::Extension for SapvcExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: SAPVC_LSP.to_string(),
            args: Vec::new(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(SapvcExtension);
