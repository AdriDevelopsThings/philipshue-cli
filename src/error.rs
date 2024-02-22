use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error while creating philipshue directory in your config directory: {0}")]
    CreatingPhilipsHueDirectory(tokio::io::Error),
    #[error("the configuration directory couldn't be found, please set the 'PHILIPSHUE_CONFIG' environment variable to a config toml file")]
    ConfigDirCouldntBeFound,
    #[error("error while parsing the configuration toml file: {0}")]
    TomlParsing(#[from] toml::de::Error),
    #[error("error while serializing the configuration toml file: {0}")]
    TomlSerializing(#[from] toml::ser::Error),
    #[error("error while writing the configuration file: {0}")]
    ConfigFileWriting(tokio::io::Error),
    #[error(
        "you aren't authorized yet, run the cli with 'login' as command to login at the hue bridge"
    )]
    NotAuthorized,
    #[error("error while communicating with the philipshue bridge: {0}")]
    PhilipsHue(#[from] philipshue::HueError),
    #[error(
        "error while discovering the hue bridge, set the '--hue-bridge' cli argument instead: {0}"
    )]
    HueBridgeDiscover(philipshue::HueError),
    #[error("no light with this light number or light name was found")]
    NoLightFound,
    #[error("link button not pressed, restart the login task after the link button was pressed with the same device type")]
    LinkButtonNotPressed,
}
