use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct Ripwire;

impl zed::Extension for Ripwire {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        // The settings override (lsp.ripwire-lsp.binary) wins when present; otherwise ripwire must
        // be on PATH. No hardcoded dev tree - a shared extension should tell you what to fix.
        let path = worktree.which("ripwire").ok_or_else(|| {
            "ripwire is not on PATH; set lsp.ripwire-lsp.binary.path in Zed settings to the ripwire binary (>= 0.6.1, built with --lsp)"
                .to_string()
        })?;
        Ok(zed::Command {
            command: path,
            args: vec!["--lsp".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(Ripwire);