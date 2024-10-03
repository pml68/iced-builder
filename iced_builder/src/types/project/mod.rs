use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::Error;

use super::rendered_element::RenderedElement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: Option<String>,
    pub content: Option<RenderedElement>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            title: None,
            content: None,
        }
    }

    pub async fn from_file() -> Result<(PathBuf, Self), Error> {
        let picked_file = rfd::AsyncFileDialog::new()
            .set_title("Open a JSON file...")
            .add_filter("*.JSON, *.json", &["JSON", "json"])
            .pick_file()
            .await
            .ok_or(Error::DialogClosed)?;

        let path = picked_file.path().to_owned();

        let contents = tokio::fs::read_to_string(&path).await?;
        let element: Self = serde_json::from_str(&contents)?;

        Ok((path, element))
    }

    pub async fn write_to_file(self, path: Option<PathBuf>) -> Result<PathBuf, Error> {
        let path = if let Some(p) = path {
            p
        } else {
            rfd::AsyncFileDialog::new()
                .set_title("Save to JSON file...")
                .add_filter("*.JSON, *.json", &["JSON", "json"])
                .save_file()
                .await
                .as_ref()
                .map(rfd::FileHandle::path)
                .map(Path::to_owned)
                .ok_or(Error::DialogClosed)?
        };

        let contents = serde_json::to_string(&self.clone())?;
        tokio::fs::write(&path, contents).await?;

        Ok(path)
    }
}
