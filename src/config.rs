// (c) 2022-2024 Cory Forsstrom, Casper Rogild Storm, Calvin Lee, Andrew Baldwin, Reza Alizadeh Majd
// (c) 2024-2025 Polesznyák Márk László

use std::path::PathBuf;

use serde::Deserialize;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReadDirStream;

use crate::theme::{Appearance, Theme, theme_from_str, theme_index};
use crate::{Error, environment};

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub theme: Appearance,
    pub last_project: Option<PathBuf>,
}

impl Config {
    pub fn selected_theme(&self) -> iced::Theme {
        self.theme.selected.clone()
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

        #[derive(Deserialize)]
        pub struct Configuration {
            #[serde(default)]
            pub theme: String,
            pub last_project: Option<PathBuf>,
        }

        let path = Self::config_file_path();
        if !path.try_exists()? {
            return Err(Error::ConfigMissing);
        }

        let content = fs::read_to_string(path).await?;

        let Configuration {
            theme,
            last_project,
        } = toml::from_str(content.as_ref())?;

        let theme = Self::load_theme(theme).await.unwrap_or_default();

        Ok(Self {
            theme,
            last_project,
        })
    }

    pub async fn load_theme(theme_name: String) -> Result<Appearance, Error> {
        use tokio::fs;

        let read_entry = async move |entry: fs::DirEntry| {
            let content = fs::read_to_string(entry.path()).await.ok()?;

            let theme: Theme = toml::from_str(content.as_ref()).ok()?;

            Some(iced::Theme::from(theme))
        };

        let mut selected = Theme::default().into();
        let mut all = iced::Theme::ALL.to_owned();
        all.push(Theme::default().into());

        if theme_index(&theme_name, iced::Theme::ALL).is_some() {
            selected = theme_from_str(None, &theme_name);
        }

        let mut stream =
            ReadDirStream::new(fs::read_dir(Self::themes_dir()).await?);
        while let Some(entry) = stream.next().await {
            let Ok(entry) = entry else {
                continue;
            };

            if let Some(theme) = read_entry(entry).await {
                if theme.to_string() == theme_name {
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
