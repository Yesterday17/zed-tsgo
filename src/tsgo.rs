use zed_extension_api as zed;

struct TsGoExtension;

impl zed::Extension for TsGoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed::Command {
            command: "tsgo".to_string(),
            args: vec!["lsp".to_string(), "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TsGoExtension);
