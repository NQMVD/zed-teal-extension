use zed_extension_api as zed;

struct TealExtension;

impl zed::Extension for TealExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // First priority: Look for the dedicated Teal language server
        if let Some(teal_ls_path) = worktree.which("teal-language-server") {
            return Ok(zed::Command {
                command: teal_ls_path,
                args: vec![
                    "--log-mode=by_proj_path".to_string(),
                    "--verbose=false".to_string(),
                ],
                env: Default::default(),
            });
        }

        // Second priority: Check if we're in the project directory with the local language server
        let root_path = worktree.root_path();
        let project_teal_ls_path =
            format!("{root_path}/teal-language-server/bin/teal-language-server");
        if std::path::Path::new(&project_teal_ls_path).exists() {
            return Ok(zed::Command {
                command: project_teal_ls_path,
                args: vec![
                    "--log-mode=by_proj_path".to_string(),
                    "--verbose=false".to_string(),
                ],
                env: Default::default(),
            });
        }

        // Third priority: Try lua_modules installation (from source build)
        if let Some(lua_modules_teal_ls) = worktree.which("lua_modules/bin/teal-language-server") {
            return Ok(zed::Command {
                command: lua_modules_teal_ls,
                args: vec![
                    "--log-mode=by_proj_path".to_string(),
                    "--verbose=false".to_string(),
                ],
                env: Default::default(),
            });
        }

        Err("Teal language server not found. Please install via 'luarocks install teal-language-server' or build from source.".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        // Teal language server doesn't require special initialization options
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        // Teal language server uses tlconfig.lua for configuration
        Ok(None)
    }
}

zed::register_extension!(TealExtension);
