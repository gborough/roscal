use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result};
use roscal_lib::{
    assessment::{
        assessment_plan::AssessmentPlanBuilder,
        assessment_results::AssessmentResultsBuilder,
        poam::PlanOfActionAndMilestonesBuilder,
    },
    control::{catalog::CatalogBuilder, profile::ProfileBuilder},
    implementation::{
        component_definition::ComponentDefinitionBuilder,
        ssp::SystemSecurityPlanBuilder,
    },
    UpdateUuid,
};
use strum::EnumString;
use tokio::fs::create_dir_all;

use crate::cli::cli_opts::Merge;

use super::{
    model::{
        AssessmentPlanBlock, AssessmentResultsBlock, CatalogBlock,
        ComponentDefinitionBlock, OscalModels, PoamBlock, ProfileBlock,
        SspBlock,
    },
    utils::{gen_created_at, gen_rand},
    workspace::{CliError, DissectCtx, MergeCtx},
};
use crate::models::utils::is_valid_dir;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum MergeOpt {
    #[strum(serialize = "json")]
    Json,
    #[strum(serialize = "yaml")]
    Yaml,
}

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum UpdateUuidOpt {
    #[strum(serialize = "v4")]
    V4,
    #[strum(serialize = "v5")]
    V5,
}

async fn gen_dir(
    path: &Option<PathBuf>,
    default: impl AsRef<Path>,
) -> Result<String> {
    if let Some(path) = path {
        if is_valid_dir(path) {
            Ok(format!(
                "{}",
                Path::new(&path).canonicalize().with_context(|| {
                    format!("Could not determine the full path of the workspace: `{}`", 
                    path.display())
                })?.to_string_lossy()
            ))
        } else {
            std::process::exit(1)
        }
    } else {
        Ok(format!(
            "{}",
            default
                .as_ref()
                .canonicalize()
                .with_context(|| {
                    format!(
                        "Could not determine the full path of the workspace: `{}`",
                        default.as_ref().display()
                    )
                })?
                .to_string_lossy()
        ))
    }
}

async fn read_dissect_ctx(path: &Option<PathBuf>) -> Result<DissectCtx> {
    let dir = gen_dir(path, "./").await?;
    let path = Path::new(&dir).join("dissect_manifest.yaml");
    let file = File::open(path).with_context(|| {
        format!(
            "Could not read dissect manifest file at this location: {}",
            &dir
        )
    })?;

    let reader = BufReader::new(file);

    let manifest: DissectCtx = serde_yaml::from_reader(reader)
        .with_context(|| "Could not determine dissect manifest content")?;

    Ok(manifest)
}

async fn gen_merge_ctx(opts: &Merge, ctx_ref: DissectCtx) -> Result<MergeCtx> {
    let created_at = gen_created_at();
    let rand = gen_rand();
    let output_dir = gen_dir(&opts.output_dir, "./").await?;

    Ok(MergeCtx {
        created_at,
        rand,
        output_dir,
        dissect_workspace_ref: ctx_ref,
        ..Default::default()
    })
}

async fn gen_merge_blocks(ctx: &MergeCtx) -> Result<Vec<PathBuf>> {
    let mut blocks: Vec<PathBuf> = vec![];
    for elt in &ctx.dissect_workspace_ref.blocks {
        let path = Path::new(&ctx.dissect_workspace_ref.modifiable)
            .join(format!("{}.yaml", elt));
        let mut file = File::open(&path).with_context(|| {
            format!(
                "Could not read workspace modifiable file at this location: {}",
                path.display()
            )
        })?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if !content.is_empty() {
            blocks.push(path)
        }
    }

    Ok(blocks)
}

async fn gen_merge_dir(
    ctx: &mut MergeCtx,
    opts: &Merge,
    model: String,
) -> Result<()> {
    create_dir_all(&ctx.output_dir).await.with_context(|| {
        format!("Could not create merge directory: `{}`", &ctx.output_dir)
    })?;

    match MergeOpt::from_str(&opts.output_format)
        .map_err(|_| CliError::UnknownMergeOpt)?
    {
        MergeOpt::Json => {
            let merged_path = format!("{}/merged.json", &ctx.output_dir);
            let mut merged = File::create(&merged_path).with_context(|| {
                format!(
                    "Could not create merged.json file at this location: {}",
                    &ctx.output_dir
                )
            })?;

            merged
                .write_all(model.as_bytes())
                .with_context(|| "Could not write to merge file".to_owned())?;
            ctx.update_hash(&model)?;
            println!("Merge Operation Successful!\nYou can locate you merged JSON file at: {}", merged_path);
        }
        MergeOpt::Yaml => {
            let merged_path = format!("{}/merged.yaml", &ctx.output_dir);
            let mut merged = File::create(&merged_path).with_context(|| {
                format!(
                    "Could not create merged.yaml file at this location: {}",
                    &ctx.output_dir
                )
            })?;
            merged
                .write_all(model.as_bytes())
                .with_context(|| "Could not write to merge file".to_owned())?;
            ctx.update_hash(&model)?;
            println!("Merge Operation Successful!\nYou can locate you merged YAML file at: {}", merged_path);
        }
    }

    let manifest_path = format!("{}/merge_manifest.yaml", &ctx.output_dir);

    let mut manifest = File::create(&manifest_path).with_context(|| {
        format!(
            "Could not create merge_manifest.yaml file at this location: {}",
            &ctx.output_dir
        )
    })?;
    let ctx_content = serde_yaml::to_string(&ctx).with_context(|| {
        "Could not parse merge_manifest.yaml file".to_owned()
    })?;

    manifest
        .write_all(ctx_content.as_bytes())
        .with_context(|| "Could not write to merge manifest".to_owned())?;
    print!("You can locate you merge manifest at: {}", manifest_path);

    Ok(())
}

async fn process_merge_blocks(
    blocks: &Vec<PathBuf>,
    ctx: &mut MergeCtx,
    opts: &Merge,
) -> Result<()> {
    let models = OscalModels::from_str(&ctx.dissect_workspace_ref.model)
        .with_context(|| {
            "Could not determine the provided OSCAL model".to_string()
        })?;
    let model_loc =
        PathBuf::from(&format!("{}/backup", ctx.dissect_workspace_ref.backup));

    match models {
        OscalModels::AssessmentPlan => {
            let assessment_plan_orig =
                models.read_assessment_plan(&model_loc).await?;
            let mut assessment_plan_sap_orig =
                assessment_plan_orig.assessment_plan.clone();
            let mut assessment_plan_builder = AssessmentPlanBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block =
                    AssessmentPlanBlock::from_str(path).with_context(|| {
                        "Could not determine the provided Assessment Plan block"
                            .to_string()
                    })?;

                match block {
                    AssessmentPlanBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        assessment_plan_sap_orig.uuid = res
                    }
                    AssessmentPlanBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        assessment_plan_sap_orig.metadata = res
                    }
                    AssessmentPlanBlock::ImportSsp => {
                        let res = block.read_import_ssp(elt).await?;
                        assessment_plan_sap_orig.import_ssp = res
                    }
                    AssessmentPlanBlock::LocalDefinitions => {
                        let res = block.read_local_definitions(elt).await?;
                        assessment_plan_sap_orig.local_definitions = Some(res)
                    }
                    AssessmentPlanBlock::TermsAndConditions => {
                        let res = block.read_terms_and_conditions(elt).await?;
                        assessment_plan_sap_orig.terms_and_conditions =
                            Some(res)
                    }
                    AssessmentPlanBlock::ReviewedControls => {
                        let res = block.read_reviewed_controls(elt).await?;
                        assessment_plan_sap_orig.reviewed_controls = res
                    }
                    AssessmentPlanBlock::AssessmentSubjects => {
                        let res = block.read_assessment_subjects(elt).await?;
                        assessment_plan_sap_orig.assessment_subjects = Some(res)
                    }
                    AssessmentPlanBlock::AssessmentAssets => {
                        let res = block.read_assessment_assets(elt).await?;
                        assessment_plan_sap_orig.assessment_assets = Some(res)
                    }
                    AssessmentPlanBlock::Tasks => {
                        let res = block.read_tasks(elt).await?;
                        assessment_plan_sap_orig.tasks = Some(res)
                    }
                    AssessmentPlanBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        assessment_plan_sap_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        assessment_plan_sap_orig.update_uuid_v4(
                            &assessment_plan_orig.assessment_plan,
                        );
                    }
                    UpdateUuidOpt::V5 => {
                        assessment_plan_sap_orig.update_uuid_v5(
                            &assessment_plan_orig.assessment_plan,
                        );
                    }
                }
            }

            let res = assessment_plan_builder
                .assessment_plan(assessment_plan_sap_orig)
                .build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model to json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model to yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
        OscalModels::AssessmentResults => {
            let assessment_results_orig =
                models.read_assessment_results(&model_loc).await?;
            let mut assessment_results_sar_orig =
                assessment_results_orig.assessment_results.clone();
            let mut assessment_results_builder =
                AssessmentResultsBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block = AssessmentResultsBlock::from_str(path).with_context(|| {
                    "Could not determine the provided Assessment Results block".to_string()
                })?;

                match block {
                    AssessmentResultsBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        assessment_results_sar_orig.uuid = res
                    }
                    AssessmentResultsBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        assessment_results_sar_orig.metadata = res
                    }
                    AssessmentResultsBlock::ImportAp => {
                        let res = block.read_import_ap(elt).await?;
                        assessment_results_sar_orig.import_ap = res
                    }
                    AssessmentResultsBlock::LocalDefinitions => {
                        let res = block.read_local_definitions(elt).await?;
                        assessment_results_sar_orig.local_definitions =
                            Some(res)
                    }
                    AssessmentResultsBlock::Results => {
                        let res = block.read_results(elt).await?;
                        assessment_results_sar_orig.results = res
                    }
                    AssessmentResultsBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        assessment_results_sar_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        assessment_results_sar_orig.update_uuid_v4(
                            &assessment_results_orig.assessment_results,
                        );
                    }
                    UpdateUuidOpt::V5 => {
                        assessment_results_sar_orig.update_uuid_v5(
                            &assessment_results_orig.assessment_results,
                        );
                    }
                }
            }

            let res = assessment_results_builder
                .assessment_results(assessment_results_sar_orig)
                .build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model to json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model to yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
        OscalModels::Poam => {
            let poam_orig = models.read_poam(&model_loc).await?;
            let mut poam_poam_orig =
                poam_orig.plan_of_action_and_milestones.clone();
            let mut poam_builder = PlanOfActionAndMilestonesBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block = PoamBlock::from_str(path).with_context(|| {
                    "Could not determine the provided Poam block".to_string()
                })?;

                match block {
                    PoamBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        poam_poam_orig.uuid = res
                    }
                    PoamBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        poam_poam_orig.metadata = res
                    }
                    PoamBlock::ImportSsp => {
                        let res = block.read_import_ssp(elt).await?;
                        poam_poam_orig.import_ssp = Some(res)
                    }
                    PoamBlock::SystemId => {
                        let res = block.read_system_id(elt).await?;
                        poam_poam_orig.system_id = Some(res)
                    }
                    PoamBlock::LocalDefinitions => {
                        let res =
                            block.read_system_local_definitions(elt).await?;
                        poam_poam_orig.local_definitions = Some(res)
                    }
                    PoamBlock::Observations => {
                        let res = block.read_observations(elt).await?;
                        poam_poam_orig.observations = Some(res)
                    }
                    PoamBlock::Risks => {
                        let res = block.read_risks(elt).await?;
                        poam_poam_orig.risks = Some(res)
                    }
                    PoamBlock::Findings => {
                        let res = block.read_findings(elt).await?;
                        poam_poam_orig.findings = Some(res)
                    }
                    PoamBlock::PoamItems => {
                        let res = block.read_poam_items(elt).await?;
                        poam_poam_orig.poam_items = res
                    }
                    PoamBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        poam_poam_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        poam_poam_orig.update_uuid_v4(
                            &poam_orig.plan_of_action_and_milestones,
                        );
                    }
                    UpdateUuidOpt::V5 => {
                        poam_poam_orig.update_uuid_v5(
                            &poam_orig.plan_of_action_and_milestones,
                        );
                    }
                }
            }

            let res = poam_builder
                .plan_of_action_and_milestones(poam_poam_orig)
                .build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model as json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model as yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
        OscalModels::Catalog => {
            let catalog_orig = models.read_catalog(&model_loc).await?;
            let mut catalog_class_orig = catalog_orig.catalog.clone();
            let mut catalog_builder = CatalogBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block =
                    CatalogBlock::from_str(path).with_context(|| {
                        "Could not determine the provided Catalog block"
                            .to_string()
                    })?;

                match block {
                    CatalogBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        catalog_class_orig.uuid = res
                    }
                    CatalogBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        catalog_class_orig.metadata = res
                    }
                    CatalogBlock::Params => {
                        let res = block.read_params(elt).await?;
                        catalog_class_orig.params = Some(res)
                    }
                    CatalogBlock::Controls => {
                        let res = block.read_controls(elt).await?;
                        catalog_class_orig.controls = Some(res)
                    }
                    CatalogBlock::Groups => {
                        let res = block.read_groups(elt).await?;
                        catalog_class_orig.groups = Some(res)
                    }
                    CatalogBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        catalog_class_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        catalog_class_orig
                            .update_uuid_v4(&catalog_orig.catalog);
                    }
                    UpdateUuidOpt::V5 => {
                        catalog_class_orig
                            .update_uuid_v5(&catalog_orig.catalog);
                    }
                }
            }

            let res = catalog_builder.catalog(catalog_class_orig).build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model to json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model to yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
        OscalModels::Profile => {
            let profile_orig = models.read_profile(&model_loc).await?;
            let mut profile_class_orig = profile_orig.profile.clone();
            let mut profile_builder = ProfileBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block =
                    ProfileBlock::from_str(path).with_context(|| {
                        "Could not determine the provided Profile block"
                            .to_string()
                    })?;

                match block {
                    ProfileBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        profile_class_orig.uuid = res
                    }
                    ProfileBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        profile_class_orig.metadata = res
                    }
                    ProfileBlock::Imports => {
                        let res = block.read_imports(elt).await?;
                        profile_class_orig.imports = res
                    }
                    ProfileBlock::Merge => {
                        let res = block.read_merge(elt).await?;
                        profile_class_orig.merge = Some(res)
                    }
                    ProfileBlock::Modify => {
                        let res = block.read_modify(elt).await?;
                        profile_class_orig.modify = Some(res)
                    }
                    ProfileBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        profile_class_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        profile_class_orig
                            .update_uuid_v4(&profile_orig.profile);
                    }
                    UpdateUuidOpt::V5 => {
                        profile_class_orig
                            .update_uuid_v5(&profile_orig.profile);
                    }
                }
            }

            let res = profile_builder.profile(profile_class_orig).build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model to json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model to yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
        OscalModels::ComponentDefinition => {
            let component_definition_orig =
                models.read_component_definition(&model_loc).await?;
            let mut component_definition_class_orig =
                component_definition_orig.component_definition.clone();
            let mut component_definition_builder =
                ComponentDefinitionBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block = ComponentDefinitionBlock::from_str(path).with_context(|| {
                    "Could not determine the provided Component Definition block".to_string()
                })?;

                match block {
                    ComponentDefinitionBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        component_definition_class_orig.uuid = res
                    }
                    ComponentDefinitionBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        component_definition_class_orig.metadata = res
                    }
                    ComponentDefinitionBlock::ImportComponentDefinitions => {
                        let res = block
                            .read_import_component_definitions(elt)
                            .await?;
                        component_definition_class_orig
                            .import_component_definitions = Some(res)
                    }
                    ComponentDefinitionBlock::Components => {
                        let res = block.read_components(elt).await?;
                        component_definition_class_orig.components = Some(res)
                    }
                    ComponentDefinitionBlock::Capabilities => {
                        let res = block.read_capabilities(elt).await?;
                        component_definition_class_orig.capabilities = Some(res)
                    }
                    ComponentDefinitionBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        component_definition_class_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        component_definition_class_orig.update_uuid_v4(
                            &component_definition_orig.component_definition,
                        );
                    }
                    UpdateUuidOpt::V5 => {
                        component_definition_class_orig.update_uuid_v5(
                            &component_definition_orig.component_definition,
                        );
                    }
                }
            }

            let res = component_definition_builder
                .component_definition(component_definition_class_orig)
                .build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model to json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model to yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
        OscalModels::Ssp => {
            let ssp_orig = models.read_ssp(&model_loc).await?;
            let mut ssp_ssp_orig = ssp_orig.system_security_plan.clone();
            let mut ssp_builder = SystemSecurityPlanBuilder::default();

            for elt in blocks {
                let path = elt.file_stem().unwrap().to_str().unwrap();
                let block = SspBlock::from_str(path).with_context(|| {
                    "Could not determine the provided System Security Plan block".to_string()
                })?;

                match block {
                    SspBlock::Uuid => {
                        let res = block.read_uuid(elt).await?;
                        ssp_ssp_orig.uuid = res
                    }
                    SspBlock::Metadata => {
                        let res = block.read_metadata(elt).await?;
                        ssp_ssp_orig.metadata = res
                    }
                    SspBlock::ImportProfile => {
                        let res = block.read_import_profile(elt).await?;
                        ssp_ssp_orig.import_profile = res
                    }
                    SspBlock::SystemCharacteristics => {
                        let res =
                            block.read_system_characteristics(elt).await?;
                        ssp_ssp_orig.system_characteristics = res
                    }
                    SspBlock::SystemImplementation => {
                        let res = block.read_system_implementation(elt).await?;
                        ssp_ssp_orig.system_implementation = res
                    }
                    SspBlock::ControlImplementation => {
                        let res =
                            block.read_control_implementation(elt).await?;
                        ssp_ssp_orig.control_implementation = res
                    }
                    SspBlock::BackMatter => {
                        let res = block.read_back_matter(elt).await?;
                        ssp_ssp_orig.back_matter = Some(res)
                    }
                    _ => {}
                }
            }

            if let Some(uuid_opt) = &opts.update_uuid {
                match UpdateUuidOpt::from_str(uuid_opt)
                    .map_err(|_| CliError::UnknownUuidVer)?
                {
                    UpdateUuidOpt::V4 => {
                        ssp_ssp_orig
                            .update_uuid_v4(&ssp_orig.system_security_plan);
                    }
                    UpdateUuidOpt::V5 => {
                        ssp_ssp_orig
                            .update_uuid_v5(&ssp_orig.system_security_plan);
                    }
                }
            }

            let res = ssp_builder.system_security_plan(ssp_ssp_orig).build()?;

            let model = match MergeOpt::from_str(&opts.output_format)
                .map_err(|_| CliError::UnknownMergeOpt)?
            {
                MergeOpt::Json => serde_json::to_string(&res)
                    .with_context(|| "Could not parse model to json file")?,
                MergeOpt::Yaml => serde_yaml::to_string(&res)
                    .with_context(|| "Could not parse model to yaml file")?,
            };

            gen_merge_dir(ctx, opts, model).await?;

            Ok(())
        }
    }
}

pub(super) async fn merge_workspace(opts: &Merge) -> Result<()> {
    let manifest = read_dissect_ctx(&opts.dir).await?;

    if manifest.is_valid_hash()? {
        let mut ctx = gen_merge_ctx(opts, manifest).await?;
        let blocks = gen_merge_blocks(&ctx).await?;
        process_merge_blocks(&blocks, &mut ctx, opts).await?;
    } else {
        eprintln!("File integrity check faild as the hash does not match the original, program aborted");
        std::process::exit(1)
    }

    Ok(())
}
