use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct RipwireLsp;

impl zed::Extension for RipwireLsp {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        // Resolution order: RIPWIRE_PATH from the user's shell environment, then `ripwire` on
        // PATH. Zed's settings override (lsp.ripwire-lsp.binary.path) still wins over both, since
        // Zed consults it before asking the extension. std::env is not available in the wasm
        // sandbox, so the shell environment is read through the worktree API.
        let from_env = worktree
            .shell_env()
            .into_iter()
            .find(|(key, value)| key == "RIPWIRE_PATH" && !value.is_empty())
            .map(|(_, value)| value);
        let path = from_env.or_else(|| worktree.which("ripwire")).ok_or_else(|| {
            "ripwire is not on PATH and RIPWIRE_PATH is not set; set lsp.ripwire-lsp.binary.path in Zed settings to the ripwire binary (>= 0.6.1, provides --lsp)"
                .to_string()
        })?;
        Ok(zed::Command {
            command: path,
            args: vec!["--lsp".to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(RipwireLsp);
