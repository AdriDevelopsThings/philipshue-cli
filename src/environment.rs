use std::{env, path::PathBuf};

use tokio::{
    fs::{create_dir, read_to_string, File},
    io::AsyncWriteExt,
};

use crate::error::Error;
use philipshue::{Hue, HueBridge};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    version: String,
    bridge_url: Option<String>,
    username: Option<String>,
}

pub struct Environment {
    config: Config,
    hue: Option<Hue>,
}

async fn get_default_config_path() -> Result<String, Error> {
    if let Some(config_dir) = dirs::config_dir() {
        let philipshue_dir = config_dir.join("philipshue");
        if !philipshue_dir.exists() {
            create_dir(&philipshue_dir)
                .await
                .map_err(Error::CreatingPhilipsHueDirectory)?;
        }
        Ok(philipshue_dir
            .join("config.toml")
            .to_str()
            .unwrap()
            .to_string())
    } else {
        Err(Error::ConfigDirCouldntBeFound)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            bridge_url: None,
            username: None,
        }
    }
}

impl Config {
    async fn get_config_path() -> Result<PathBuf, Error> {
        Ok(PathBuf::from(match env::var("PHILIPSHUE_CONFIG") {
            Ok(path) => path,
            Err(_) => get_default_config_path().await?,
        }))
    }

    async fn new() -> Result<Self, Error> {
        let config_path = Self::get_config_path().await?;
        if config_path.exists() {
            let content = read_to_string(config_path)
                .await
                .map_err(Error::ConfigFileWriting)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    async fn save(&self) -> Result<(), Error> {
        let mut file = File::options()
            .write(true)
            .create(true)
            .open(Self::get_config_path().await?)
            .await
            .map_err(Error::ConfigFileWriting)?;
        let toml = toml::to_string(self)?;
        file.write_all(toml.as_bytes())
            .await
            .map_err(Error::ConfigFileWriting)?;
        Ok(())
    }
}

impl Environment {
    pub async fn new() -> Result<Self, Error> {
        let mut environment = Environment {
            config: Config::new().await?,
            hue: None,
        };
        if let Some(bridge_url) = &environment.config.bridge_url {
            if let Some(username) = &environment.config.username {
                environment.hue = Some(Hue::new(
                    HueBridge::new(bridge_url.clone()),
                    username.clone(),
                ));
            }
        }
        Ok(environment)
    }

    pub fn get_hue(&self) -> Result<&Hue, Error> {
        self.hue.as_ref().ok_or(Error::NotAuthorized)
    }

    pub async fn set_hue(&mut self, hue: Hue) -> Result<(), Error> {
        self.hue = Some(hue.clone());
        self.config.bridge_url = Some(hue.bridge.bridge_url);
        self.config.username = Some(hue.username);
        self.config.save().await?;
        Ok(())
    }
}
