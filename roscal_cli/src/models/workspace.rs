use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::cli::cli_opts::{Dissect, Merge, Validate};

use super::{
    dissect::dissect_workspace, merge::merge_workspace, utils::*,
    validate::validate_model,
};

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Could not parse {0} model from file: {1}\nCause ---> {2}")]
    ParseModel(String, String, String),
    #[error("Could not parse {0} field for model {1} from file: {2}\nCause ---> {3}")]
    ParseBlock(String, String, String, String),
    #[error("Unknown output format. Use json or yaml")]
    UnknownMergeOpt,
    #[error("Unknown uuid version. Use v4 or v5")]
    UnknownUuidVer,
}

#[derive(Debug, Clone)]
pub struct Workspace;

impl Workspace {
    pub async fn dissect(
        opts: &Dissect,
    ) -> Result<(), Box<dyn std::error::Error>> {
        dissect_workspace(opts).await?;

        Ok(())
    }

    pub async fn merge(opts: &Merge) -> Result<(), Box<dyn std::error::Error>> {
        merge_workspace(opts).await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Validator;

impl Validator {
    pub async fn validate(
        opts: &Validate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        validate_model(opts).await?;

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DissectCtx {
    pub created_at: String,
    pub model_loc: PathBuf,
    pub model: String,
    pub blocks: Vec<String>,
    pub rand: String,
    pub root: String,
    pub backup: String,
    pub modifiable: String,
    pub hash: String,
}

impl DissectCtx {
    pub fn is_valid_hash(&self) -> Result<bool> {
        let path = format!("{}/backup", &self.backup);
        let hash = gen_hash_from_path(&path);

        Ok(hash.is_ok() && hash? == self.hash)
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MergeCtx {
    pub created_at: String,
    pub hash: String,
    pub rand: String,
    pub output_dir: String,
    pub dissect_workspace_ref: DissectCtx,
}

impl MergeCtx {
    pub fn update_hash(&mut self, content: &str) -> Result<()> {
        let hash = gen_hash_from_str(content)?;
        self.hash = hash;

        Ok(())
    }
}
