use zed_extension_api as zed;

struct TealExtension;

impl zed::Extension for TealExtension {
    fn new() -> Self {
        println!("🔧 TealExtension::new() called - extension is being initialized");
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        println!("🔧 TealExtension::language_server_command() called for language server: {:?}", _language_server_id);
        
        // For now, we'll use the Lua language server as a fallback
        // since Teal is a typed dialect of Lua
        if let Some(lua_ls_path) = worktree.which("lua-language-server") {
            println!("🔧 Found lua-language-server at: {}", lua_ls_path);
            return Ok(zed::Command {
                command: lua_ls_path,
                args: vec![],
                env: Default::default(),
            });
        }

        // Try to find tl compiler for basic functionality
        if let Some(_tl_path) = worktree.which("tl") {
            println!("🔧 Found tl compiler at: {}", _tl_path);
            // Note: tl doesn't have LSP mode, but we can still reference it
            // for potential future integration or custom LSP wrapper
            return Err("Teal language server not yet implemented. Consider using Lua LSP as fallback.".to_string());
        }

        println!("🔧 No language server found for Teal");
        Err("Neither Teal compiler 'tl' nor Lua language server found. Please install via 'luarocks install tl' or install lua-language-server.".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        println!("🔧 TealExtension::language_server_initialization_options() called");
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        println!("🔧 TealExtension::language_server_workspace_configuration() called");
        Ok(None)
    }
}

zed::register_extension!(TealExtension);