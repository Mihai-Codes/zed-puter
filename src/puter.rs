use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

// The npm package that contains the MCP server
// TODO: Publish puter-mcp-server to npm when ready
const PACKAGE_NAME: &str = "opencode-puter-auth";
const PACKAGE_VERSION: &str = "1.1.1";
const SERVER_PATH: &str = "node_modules/opencode-puter-auth/dist/mcp-server.js";

/// Settings for the Puter MCP extension
#[derive(Debug, Deserialize, JsonSchema, Default)]
struct PuterExtensionSettings {
    /// Default model to use (e.g., "claude-opus-4-5", "gpt-4o")
    #[serde(default)]
    default_model: Option<String>,

    /// Enable fallback to free models when rate limited
    #[serde(default = "default_true")]
    fallback_enabled: Option<bool>,

    /// Enable debug logging
    #[serde(default)]
    debug: Option<bool>,
}

fn default_true() -> Option<bool> {
    Some(true)
}

struct PuterExtension;

impl zed::Extension for PuterExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Check if npm package is installed, install if needed
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        // Get user settings
        let settings = ContextServerSettings::for_project("puter", project)?;
        let settings_struct: PuterExtensionSettings = match settings.settings {
            Some(v) => serde_json::from_value(v).map_err(|e| e.to_string())?,
            None => PuterExtensionSettings::default(),
        };

        // Build command arguments
        let mut args = Vec::new();
        args.push(
            env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string(),
        );

        // Pass settings as arguments
        if let Some(model) = settings_struct.default_model {
            args.push(format!("--model={}", model));
        }
        if settings_struct.fallback_enabled == Some(false) {
            args.push("--no-fallback".to_string());
        }
        if settings_struct.debug == Some(true) {
            args.push("--debug".to_string());
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: Default::default(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("puter", project);

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(puter_settings) =
                    serde_json::from_value::<PuterExtensionSettings>(settings_value)
                {
                    if let Some(model) = puter_settings.default_model {
                        default_settings = default_settings
                            .replace("\"claude-opus-4-5\"", &format!("\"{}\"", model));
                    }
                }
            }
        }

        let settings_schema = serde_json::to_string(&schemars::schema_for!(PuterExtensionSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(PuterExtension);
