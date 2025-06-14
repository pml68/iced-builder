// (c) 2022-2024 Cory Forsstrom, Casper Rogild Storm, Calvin Lee, Andrew Baldwin, Reza Alizadeh Majd
// (c) 2024-2025 Polesznyák Márk László

use std::path::{Path, PathBuf};
use std::sync::Arc;

use material_theme::Theme;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReadDirStream;

use crate::appearance::Appearance;
use crate::{Error, environment};

#[derive(Debug, Clone, Default)]
pub struct Config {
    appearance: Appearance,
    last_project: Option<PathBuf>,
}

impl Config {
    pub fn selected_theme(&self) -> Theme {
        self.appearance.selected.clone()
    }

    pub fn themes(&self) -> Arc<[Theme]> {
        self.appearance.all.clone()
    }

    pub fn last_project(&self) -> Option<&Path> {
        self.last_project.as_deref()
    }

    pub fn config_dir() -> PathBuf {
        let dir = environment::config_dir();

        if !dir.exists() {
            std::fs::create_dir_all(dir.as_path())
                .expect("expected permissions to create config folder");
        }
        dir
    }

    pub fn themes_dir() -> PathBuf {
        let dir = Self::config_dir().join("themes");

        if !dir.exists() {
            std::fs::create_dir_all(dir.as_path())
                .expect("expected permissions to create themes folder");
        }
        dir
    }

    pub fn config_file_path() -> PathBuf {
        Self::config_dir().join(environment::CONFIG_FILE_NAME)
    }

    pub async fn load() -> Result<Self, Error> {
        use tokio::fs;
        use tokio::io::AsyncWriteExt;

        #[derive(Deserialize, Serialize)]
        pub struct Configuration {
            #[serde(default)]
            pub theme: String,
            pub last_project: Option<PathBuf>,
        }

        let path = Self::config_file_path();
        if !path.try_exists()? {
            let mut config = fs::File::create(path)
                .await
                .expect("expected permissions to create config file");
            let default_config = Configuration {
                theme: Theme::default().to_string(),
                last_project: None,
            };

            config
                .write_all(
                    toml::to_string_pretty(&default_config)
                        .expect("stringify default configuration")
                        .as_bytes(),
                )
                .await
                .expect("expected config write operation to succeed");

            return Err(Error::ConfigMissing);
        }

        let content = fs::read_to_string(path).await?;

        let Configuration {
            theme,
            last_project,
        } = toml::from_str(content.as_ref())?;

        let appearance =
            Self::load_appearance(&theme).await.unwrap_or_default();

        Ok(Self {
            appearance,
            last_project,
        })
    }

    pub async fn load_appearance(
        theme_name: &str,
    ) -> Result<Appearance, Error> {
        use tokio::fs;

        let read_entry = async move |entry: fs::DirEntry| {
            let content = fs::read_to_string(entry.path()).await.ok()?;

            let theme: Theme = toml::from_str(content.as_ref()).ok()?;

            Some(theme)
        };

        let mut selected = Theme::default();
        let mut all = Theme::ALL.to_owned();

        if let Some(index) =
            Theme::ALL.iter().position(|t| t.name() == theme_name)
        {
            selected = Theme::ALL[index].clone();
        }

        let mut stream =
            ReadDirStream::new(fs::read_dir(Self::themes_dir()).await?);
        while let Some(entry) = stream.next().await {
            let Ok(entry) = entry else {
                continue;
            };

            if let Some(theme) = read_entry(entry).await {
                if theme.name() == theme_name {
                    selected = theme.clone();
                }
                all.push(theme);
            }
        }

        Ok(Appearance {
            selected,
            all: all.into(),
        })
    }
}
