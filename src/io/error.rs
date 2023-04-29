/// A result obtained from parsing YAML files. An `Ok(_)` variant contains an
/// [`Option<Camera>`](crate::core::Camera) and a [World](crate::core::World). An `Err(_)` variant
/// contains a [YamlError](crate::io::error::YamlError).
pub type ParseResult<C, W> = Result<(Option<C>, W), YamlError>;

// pub type RtcResult = Result<(), RtcError>;
pub type RtcResult<T> = anyhow::Result<T>;

/// Possible errors encountered when attempting to construct world data from YAML files.
#[derive(thiserror::Error, Debug)]
pub enum YamlError {
    /// Standard library IO error.
    IO(std::io::Error),

    /// A scanning error reported by [yaml_rust](yaml_rust::ScanError).
    Scan(yaml_rust::ScanError),
}

#[derive(thiserror::Error, Debug)]
pub enum RtcError {
    #[error("Invalid YAML file")]
    InvalidYaml(YamlError),
}

#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("Could not render the specified scene")]
    SceneError(String),
}

impl From<std::io::Error> for YamlError {
    fn from(e: std::io::Error) -> Self {
        YamlError::IO(e)
    }
}

impl From<yaml_rust::ScanError> for YamlError {
    fn from(e: yaml_rust::ScanError) -> Self {
        YamlError::Scan(e)
    }
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
