use std::path::PathBuf;

use serde::Deserialize;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;

use crate::theme::{theme_from_str, theme_index, Appearance, Theme};
use crate::{environment, Error, Result};

#[derive(Debug, Default)]
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

    pub async fn load() -> Result<Self> {
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

    pub async fn load_theme(theme_name: String) -> Result<Appearance> {
        use tokio::fs;

        let read_entry = |entry: fs::DirEntry| async move {
            let content = fs::read_to_string(entry.path()).await.ok()?;

            let theme: Theme = toml::from_str(content.as_ref()).ok()?;
            let name = entry.path().file_stem()?.to_string_lossy().to_string();

            Some(theme.into_iced_theme(name))
        };

        let mut all = iced::Theme::ALL.to_owned();
        let mut selected = iced::Theme::default();

        if theme_index(theme_name.clone(), iced::Theme::ALL).is_some() {
            selected = theme_from_str(None, &theme_name);
        }

        let mut stream =
            ReadDirStream::new(fs::read_dir(Self::themes_dir()).await?);
        while let Some(entry) = stream.next().await {
            let Ok(entry) = entry else {
                continue;
            };

            let Some(file_name) = entry.file_name().to_str().map(String::from)
            else {
                continue;
            };

            if let Some(file_name) = file_name.strip_suffix(".toml") {
                if let Some(theme) = read_entry(entry).await {
                    if file_name == theme_name {
                        selected = theme.clone();
                    }
                    all.push(theme);
                }
            }
        }

        Ok(Appearance {
            selected,
            all: all.into(),
        })
    }
}
