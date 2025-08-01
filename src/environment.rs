// (c) 2023-2025 Cory Forsstrom, Casper Rogild Storm
// (c) 2024-2025 Polesznyák Márk László

use std::env;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "config.toml";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

pub fn formatted_version() -> String {
    let hash = GIT_HASH
        .map(|hash| format!(" ({hash})"))
        .unwrap_or_default();

    format!("{}{hash}", VERSION)
}

pub fn config_dir() -> PathBuf {
    portable_dir().unwrap_or_else(platform_specific_config_dir)
}

fn portable_dir() -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    let dir = exe.parent()?;

    dir.join(CONFIG_FILE_NAME)
        .is_file()
        .then(|| dir.to_path_buf())
}

fn platform_specific_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        xdg_config_dir().unwrap_or_else(|| {
            dirs_next::config_dir()
                .expect("expected valid config dir")
                .join("iced-builder")
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        dirs_next::config_dir()
            .expect("expected valid config dir")
            .join("iced-builder")
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
fn xdg_config_dir() -> Option<PathBuf> {
    let config_path = xdg::BaseDirectories::new().config_home?;
    let config_dir = config_path.join("iced-builder");

    config_dir
        .join(CONFIG_FILE_NAME)
        .is_file()
        .then_some(config_dir)
}
