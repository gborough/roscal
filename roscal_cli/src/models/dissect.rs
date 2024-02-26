use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result};
use tokio::{
    fs::{create_dir_all, remove_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::cli::cli_opts::Dissect;

use super::{
    model::{
        AssessmentPlanBlock, AssessmentResultsBlock, CatalogBlock,
        ComponentDefinitionBlock, OscalModels, PoamBlock, ProfileBlock,
        SspBlock,
    },
    utils::{
        gen_created_at, gen_hash_from_path, gen_rand, is_valid_dir,
        is_valid_file,
    },
    validation::{
        is_valid_assessment_plan_block, is_valid_assessment_results_block,
        is_valid_catalog_block, is_valid_component_definition_block,
        is_valid_model, is_valid_poam_block, is_valid_profile_block,
        is_valid_ssp_block,
    },
    workspace::DissectCtx,
};

async fn gen_dissect_root(
    created_at: &str,
    rand: &str,
    path: &impl AsRef<Path>,
    output_dir: &Option<PathBuf>,
) -> Result<String> {
    let dirname = if is_valid_file(path) {
        format!(
            "{}_{created_at}_{rand}",
            path.as_ref().file_name().unwrap().to_string_lossy()
        )
    } else {
        std::process::exit(1)
    };

    if let Some(dir) = output_dir {
        if is_valid_dir(dir) {
            return Ok(format!(
                "{}/{}",
                dir.canonicalize()
                    .with_context(|| {
                        format!(
                            "Could not determine the output path: `{}`",
                            dir.display()
                        )
                    })?
                    .to_string_lossy(),
                dirname
            ));
        }
    }

    let cur_dir = format!(
        "{}/{}",
        PathBuf::from("./")
            .canonicalize()?
            .to_string_lossy()
            .into_owned(),
        dirname
    );

    Ok(cur_dir)
}

async fn gen_dissect_backup(root: &String) -> String {
    format!("{}/backup", root)
}

async fn gen_dissect_modifiable(root: &String) -> String {
    format!("{}/modifiable", root)
}

async fn gen_blocks(opts: &Dissect) -> Result<Vec<String>> {
    let mut blocks_in = opts.blocks.clone();
    blocks_in.dedup();
    let mut blocks_out: Vec<String> = vec![];

    if is_valid_model(&opts.model) {
        let models = OscalModels::from_str(&opts.model).with_context(|| {
            "Could not determine the provided OSCAL model".to_string()
        })?;

        match models {
            OscalModels::AssessmentPlan => {
                if is_valid_assessment_plan_block(&blocks_in) {
                    if blocks_in[0] == AssessmentPlanBlock::All.to_string() {
                        let all = vec![
                            AssessmentPlanBlock::Uuid.to_string(),
                            AssessmentPlanBlock::Metadata.to_string(),
                            AssessmentPlanBlock::ImportSsp.to_string(),
                            AssessmentPlanBlock::LocalDefinitions.to_string(),
                            AssessmentPlanBlock::TermsAndConditions.to_string(),
                            AssessmentPlanBlock::ReviewedControls.to_string(),
                            AssessmentPlanBlock::AssessmentSubjects.to_string(),
                            AssessmentPlanBlock::AssessmentAssets.to_string(),
                            AssessmentPlanBlock::Tasks.to_string(),
                            AssessmentPlanBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
            OscalModels::AssessmentResults => {
                if is_valid_assessment_results_block(&blocks_in) {
                    if blocks_in[0] == AssessmentResultsBlock::All.to_string() {
                        let all = vec![
                            AssessmentResultsBlock::Uuid.to_string(),
                            AssessmentResultsBlock::Metadata.to_string(),
                            AssessmentResultsBlock::ImportAp.to_string(),
                            AssessmentResultsBlock::LocalDefinitions
                                .to_string(),
                            AssessmentResultsBlock::Results.to_string(),
                            AssessmentResultsBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
            OscalModels::Poam => {
                if is_valid_poam_block(&blocks_in) {
                    if blocks_in[0] == PoamBlock::All.to_string() {
                        let all = vec![
                            PoamBlock::Uuid.to_string(),
                            PoamBlock::Metadata.to_string(),
                            PoamBlock::ImportSsp.to_string(),
                            PoamBlock::SystemId.to_string(),
                            PoamBlock::LocalDefinitions.to_string(),
                            PoamBlock::Observations.to_string(),
                            PoamBlock::Risks.to_string(),
                            PoamBlock::Findings.to_string(),
                            PoamBlock::PoamItems.to_string(),
                            PoamBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
            OscalModels::Catalog => {
                if is_valid_catalog_block(&blocks_in) {
                    if blocks_in[0] == CatalogBlock::All.to_string() {
                        let all = vec![
                            CatalogBlock::Uuid.to_string(),
                            CatalogBlock::Metadata.to_string(),
                            CatalogBlock::Params.to_string(),
                            CatalogBlock::Controls.to_string(),
                            CatalogBlock::Groups.to_string(),
                            CatalogBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
            OscalModels::Profile => {
                if is_valid_profile_block(&blocks_in) {
                    if blocks_in[0] == ProfileBlock::All.to_string() {
                        let all = vec![
                            ProfileBlock::Uuid.to_string(),
                            ProfileBlock::Metadata.to_string(),
                            ProfileBlock::Imports.to_string(),
                            ProfileBlock::Merge.to_string(),
                            ProfileBlock::Modify.to_string(),
                            ProfileBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
            OscalModels::ComponentDefinition => {
                if is_valid_component_definition_block(&blocks_in) {
                    if blocks_in[0] == ComponentDefinitionBlock::All.to_string()
                    {
                        let all = vec![
                            ComponentDefinitionBlock::Uuid.to_string(),
                            ComponentDefinitionBlock::Metadata.to_string(),
                            ComponentDefinitionBlock::ImportComponentDefinitions.to_string(),
                            ComponentDefinitionBlock::Components.to_string(),
                            ComponentDefinitionBlock::Capabilities.to_string(),
                            ComponentDefinitionBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
            OscalModels::Ssp => {
                if is_valid_ssp_block(&blocks_in) {
                    if blocks_in[0] == SspBlock::All.to_string() {
                        let all = vec![
                            SspBlock::Uuid.to_string(),
                            SspBlock::Metadata.to_string(),
                            SspBlock::ImportProfile.to_string(),
                            SspBlock::SystemCharacteristics.to_string(),
                            SspBlock::SystemImplementation.to_string(),
                            SspBlock::ControlImplementation.to_string(),
                            SspBlock::BackMatter.to_string(),
                        ];
                        blocks_out.extend(all);

                        Ok(blocks_out)
                    } else {
                        Ok(blocks_in)
                    }
                } else {
                    std::process::exit(1)
                }
            }
        }
    } else {
        std::process::exit(1)
    }
}

async fn gen_dissect_files(ctx: &DissectCtx) -> Result<()> {
    let models = OscalModels::from_str(&ctx.model).with_context(|| {
        "Could not determine the provided OSCAL model".to_string()
    })?;

    match models {
        OscalModels::AssessmentPlan => {
            if is_valid_assessment_plan_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
        OscalModels::AssessmentResults => {
            if is_valid_assessment_results_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
        OscalModels::Poam => {
            if is_valid_poam_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
        OscalModels::Catalog => {
            if is_valid_catalog_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
        OscalModels::Profile => {
            if is_valid_profile_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
        OscalModels::ComponentDefinition => {
            if is_valid_component_definition_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
        OscalModels::Ssp => {
            if is_valid_ssp_block(&ctx.blocks) {
                OscalModels::gen_models(&models, ctx).await?;

                Ok(())
            } else {
                std::process::exit(1)
            }
        }
    }
}

async fn dissect_cleanup(ctx: &DissectCtx) -> Result<()> {
    remove_dir_all(&ctx.root).await.with_context(|| {
      format!(
          "Could not clean up folder\nReason: {}\nPlease try manually deleting it",
          &ctx.root
      )
  })?;

    Ok(())
}

pub(super) async fn gen_dissect_dir(ctx: &DissectCtx) -> Result<()> {
    create_dir_all(&ctx.backup).await.with_context(|| {
        format!("Could not create directory: `{}`", &ctx.backup)
    })?;

    create_dir_all(&ctx.modifiable).await.with_context(|| {
        format!("Could not create directory: `{}`", &ctx.modifiable)
    })?;

    let mut instruction = File::create(format!("{}/instruction", &ctx.root))
        .await
        .with_context(|| {
            format!(
                "Could not create instruction.yaml file at this location: {}",
                &ctx.root
            )
        })?;

    let content = r#"Directory and File Structure
  - modifiable: You should only view and modified your specified blocks content here, please do not modify file name
  - backup: A back-up of the original model file for integrity check, please do not modify
  - manifest.yaml: This file contains some metadata about yor workspace, please do not modify
      "#;

    instruction
        .write_all(content.as_bytes())
        .await
        .with_context(|| "Could not write to backup YAML file".to_owned())?;

    let mut manifest =
        File::create(format!("{}/dissect_manifest.yaml", &ctx.root))
            .await
            .with_context(|| {
                format!(
                    "Could not create manifest.yaml file at this location: {}",
                    &ctx.root
                )
            })?;
    let ctx_content = serde_yaml::to_string(&ctx)
        .with_context(|| "Could not parse manifest.yaml file".to_owned())?;

    let mut manifest_extra = "# Please do not modify this file\n".to_owned();

    manifest_extra.push_str(&ctx_content);

    manifest
        .write_all(manifest_extra.as_bytes())
        .await
        .with_context(|| "Could not write to manifest".to_owned())?;

    let mut backup = File::create(format!("{}/backup", &ctx.backup))
        .await
        .with_context(|| {
            format!(
                "Could not create backup file at this location: {}",
                &ctx.backup
            )
        })?;

    let mut file = File::open(&ctx.model_loc).await.with_context(|| {
        format!(
            "Could not read backup file at this location: {}",
            &ctx.model_loc.display()
        )
    })?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .await
        .with_context(|| "Could not write to backup file".to_string())?;

    backup
        .write_all(content.as_bytes())
        .await
        .with_context(|| "Could not write to backup YAML file".to_owned())?;

    Ok(())
}

pub(super) async fn dissect_workspace(opts: &Dissect) -> Result<()> {
    if opts.parse_markup {
        std::env::set_var("OSCAL_MARKUP_RENDER_MODE", "ENABLED")
    } else {
        std::env::set_var("OSCAL_MARKUP_RENDER_MODE", "DISABLED")
    }

    let created_at = gen_created_at();
    let model_loc =
        PathBuf::from(&opts.file).canonicalize().with_context(|| {
            format!(
                "Could not determine the full path of the OSCAL model file: `{}`",
                &opts.file.display()
            )
        })?;
    let rand = gen_rand();
    let root =
        gen_dissect_root(&created_at, &rand, &opts.file, &opts.output_dir)
            .await?;
    let backup = gen_dissect_backup(&root).await;
    let modifiable = gen_dissect_modifiable(&root).await;
    let hash = gen_hash_from_path(&opts.file)?;
    let blocks = gen_blocks(opts).await?;
    let ctx = DissectCtx {
        created_at,
        model_loc,
        model: opts.model.clone(),
        blocks,
        rand,
        root,
        backup,
        modifiable,
        hash,
    };

    if let Err(e) = gen_dissect_files(&ctx).await {
        eprintln!("{e}");
        dissect_cleanup(&ctx).await?;
        std::process::exit(1)
    } else {
        println!("Dissect operation successful!\nYou can locate your workspace at: {}", &ctx.root)
    }

    Ok(())
}
