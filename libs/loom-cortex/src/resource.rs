use std::path::PathBuf;

use rust_bert::pipelines::common::ModelResource;
use rust_bert::resources::{LocalResource, RemoteResource, ResourceProvider};
use serde::{Deserialize, Serialize};

/// Serializable resource specification for model files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CortexResource {
    /// Load from a local file path
    Local { path: PathBuf },
    /// Download from a remote URL (cached locally)
    Remote { name: String, url: String },
}

impl CortexResource {
    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local { path: path.into() }
    }

    pub fn remote(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self::Remote {
            name: name.into(),
            url: url.into(),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local { .. })
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Self::Remote { .. })
    }

    pub fn into_provider(self) -> Box<dyn ResourceProvider + Send> {
        match self {
            Self::Local { path } => Box::new(LocalResource::from(path)),
            Self::Remote { name, url } => Box::new(RemoteResource::from_pretrained((
                name.as_str(),
                url.as_str(),
            ))),
        }
    }

    pub fn into_model_resource(self) -> ModelResource {
        ModelResource::Torch(self.into_provider())
    }
}

/// Simplified model source - either use defaults or specify custom resources
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CortexModelSource {
    /// Use HuggingFace defaults for the model type
    #[default]
    Default,
    /// Specify all resources explicitly
    Custom {
        model: CortexResource,
        config: CortexResource,
        vocab: CortexResource,
        #[serde(default)]
        merges: Option<CortexResource>,
    },
    /// Load from a local directory (assumes standard HuggingFace file names)
    LocalDir {
        path: PathBuf,
        #[serde(default)]
        has_merges: bool,
    },
}

impl CortexModelSource {
    pub fn local_dir(path: impl Into<PathBuf>) -> Self {
        Self::LocalDir {
            path: path.into(),
            has_merges: false,
        }
    }

    pub fn local_dir_with_merges(path: impl Into<PathBuf>) -> Self {
        Self::LocalDir {
            path: path.into(),
            has_merges: true,
        }
    }

    pub fn custom(
        model: CortexResource,
        config: CortexResource,
        vocab: CortexResource,
        merges: Option<CortexResource>,
    ) -> Self {
        Self::Custom {
            model,
            config,
            vocab,
            merges,
        }
    }

    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }

    pub fn is_local_dir(&self) -> bool {
        matches!(self, Self::LocalDir { .. })
    }

    /// Expand a local directory into individual resource specs
    pub fn expand(self) -> Self {
        match self {
            Self::LocalDir { path, has_merges } => Self::Custom {
                model: CortexResource::local(path.join("rust_model.ot")),
                config: CortexResource::local(path.join("config.json")),
                vocab: CortexResource::local(path.join("vocab.txt")),
                merges: if has_merges {
                    Some(CortexResource::local(path.join("merges.txt")))
                } else {
                    None
                },
            },
            other => other,
        }
    }
}
