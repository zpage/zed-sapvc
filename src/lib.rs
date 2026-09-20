//! Zed extension shell for SAP VC/AVC dependency files.
//!
//! The heavy lifting lives in the `sapvc-lsp` binary (tower-lsp + tree-sitter).
//! This shell only tells Zed how to spawn it, per the official extension API.
//!
//! Build: cargo build --release --target wasm32-wasip2
//! Output: target/wasm32-wasip2/release/zed-sapvc.wasm -> extension.wasm
//!
//! Configuration (environment variables):
//!   SAPVC_LSP_BIN   path to the sapvc-lsp binary
//!   SAPVC_MATERIAL  path to a material data package JSON

use zed_extension_api as zed;

const DEFAULT_LSP_BIN: &str = "sapvc-lsp";

struct SapvcExtension;

impl zed::Extension for SapvcExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let env = worktree.shell_env();
        let bin = env
            .iter()
            .find(|(k, _)| k == "SAPVC_LSP_BIN")
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| DEFAULT_LSP_BIN.to_string());

        let mut args = Vec::new();
        if let Some((_, material)) = env.iter().find(|(k, _)| k == "SAPVC_MATERIAL") {
            args.push("--data".to_string());
            args.push(material.clone());
        }

        Ok(zed::Command {
            command: bin,
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(SapvcExtension);
