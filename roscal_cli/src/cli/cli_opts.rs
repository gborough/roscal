use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(author = "Author: Geoffrey Borough<Geoffrey.Borough@outlook.com>")]
#[clap(version)]
#[clap(about = "Cli Tool For Processing and Manipulating OSCAL Model File")]
pub struct OscalCli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[non_exhaustive]
#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Dissect OSCAL model file and generate a workspace for viewing and editing
    /// Full Example:
    /// roscal dissect --file /dir/catalog.json
    /// --model Catalog
    /// --blocks controls,groups
    /// --output-dir /home/workspace
    /// --parse-markup
    #[clap(verbatim_doc_comment)]
    Dissect(#[clap(flatten)] Dissect),
    /// Merge existing worspace and generate new OSCAL model file
    /// Full Example:
    /// roscal merge --dir /dir/existing_workspace
    /// --output-dir /dir/merged
    /// --output-format yaml
    /// --update-uuid v4
    #[clap(verbatim_doc_comment)]
    Merge(#[clap(flatten)] Merge),
    /// Validate a specific type of OSCAL model file
    /// Full Example:
    /// roscal validate --file /dir/catalog.json
    /// --model Catalog
    #[clap(verbatim_doc_comment)]
    Validate(#[clap(flatten)] Validate),
    /// Show available models and blocks for dissect operation
    ShowDissect,
}

#[derive(Args, Debug, Default, Clone)]
pub struct Dissect {
    /// Location of the OSCAL model file
    #[arg(long)]
    pub file: PathBuf,
    /// Specifiy which OSCAL model to be processed
    /// Run `roscal show-dissect` for available models
    #[arg(long, verbatim_doc_comment)]
    pub model: String,
    /// Specifiy which blocks to be dissected
    /// Duplicate blocks will be combined
    /// Run `roscal show-dissect` for available blocks
    #[arg(long, num_args = 1.., value_delimiter = ',', verbatim_doc_comment)]
    pub blocks: Vec<String>,
    /// Specify where dissect workspace should be created
    /// Optional. Will use current directory if unspecified
    #[arg(long, verbatim_doc_comment)]
    pub output_dir: Option<PathBuf>,
    /// Whether to parse markup lines
    /// Currently experimental feature
    /// see https://pages.nist.gov/metaschema/specification/datatypes/#markup-data-types
    #[arg(long, verbatim_doc_comment)]
    pub parse_markup: bool,
}

#[derive(Args, Debug, Default, Clone)]
pub struct Merge {
    /// Location of existing workspace created by dissect operation
    /// Optional. Can be run directly in existing workspace
    #[arg(long, verbatim_doc_comment)]
    pub dir: Option<PathBuf>,
    /// Specify where merged file should be created
    /// Optional. Can be run directly in existing workspace
    #[arg(long, verbatim_doc_comment)]
    pub output_dir: Option<PathBuf>,
    /// Options: json or yaml
    #[arg(long)]
    pub output_format: String,
    /// Options: v4 or v5 (as in uuid version)
    /// Optional. No-op if model unchanged or uuid manually updated
    #[arg(long, verbatim_doc_comment)]
    pub update_uuid: Option<String>,
}

#[derive(Args, Debug, Default, Clone)]
pub struct Validate {
    /// Location of OSCAL model file
    #[arg(long)]
    pub file: PathBuf,
    /// Model type of OSCAL model file
    /// Run `roscal show-dissect` for available models
    #[arg(long, verbatim_doc_comment)]
    pub model: String,
}
