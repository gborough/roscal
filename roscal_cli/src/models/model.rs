use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Context, Result};
use roscal_lib::{
    assessment::{
        assessment_plan::AssessmentPlan, assessment_results::AssessmentResults,
        poam::PlanOfActionAndMilestones,
    },
    control::{catalog::Catalog, profile::Profile},
    implementation::{
        component_definition::ComponentDefinition, ssp::SystemSecurityPlan,
    },
};
use strum::{Display, EnumString};

use super::{
    dissect::gen_dissect_dir,
    workspace::{CliError, DissectCtx},
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString)]
pub(super) enum OscalModels {
    #[strum(serialize = "AssessmentPlan")]
    AssessmentPlan,
    #[strum(serialize = "AssessmentResults")]
    AssessmentResults,
    #[strum(serialize = "Poam")]
    Poam,
    #[strum(serialize = "Catalog")]
    Catalog,
    #[strum(serialize = "Profile")]
    Profile,
    #[strum(serialize = "ComponentDefinition")]
    ComponentDefinition,
    #[strum(serialize = "Ssp")]
    Ssp,
}

impl OscalModels {
    async fn read_file(path: &PathBuf) -> Result<BufReader<File>> {
        let file = File::open(path).with_context(|| {
            format!("Could not open model file: `{}`", path.display())
        })?;

        Ok(BufReader::new(file))
    }

    pub async fn read_assessment_plan(
        &self,
        path: &PathBuf,
    ) -> Result<AssessmentPlan> {
        let reader = Self::read_file(path).await?;
        let assessment_plan: AssessmentPlan = serde_yaml::from_reader(reader)
            .map_err(|e| {
            CliError::ParseModel(
                "assessment plan".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(assessment_plan)
    }

    pub async fn read_assessment_results(
        &self,
        path: &PathBuf,
    ) -> Result<AssessmentResults> {
        let reader = Self::read_file(path).await?;
        let assessment_results: AssessmentResults =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseModel(
                    "assessment results".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(assessment_results)
    }

    pub async fn read_poam(
        &self,
        path: &PathBuf,
    ) -> Result<PlanOfActionAndMilestones> {
        let reader = Self::read_file(path).await?;
        let poam: PlanOfActionAndMilestones = serde_yaml::from_reader(reader)
            .map_err(|e| {
            CliError::ParseModel(
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(poam)
    }

    pub async fn read_catalog(&self, path: &PathBuf) -> Result<Catalog> {
        let reader = Self::read_file(path).await?;
        let catalog: Catalog =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseModel(
                    "catalog".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(catalog)
    }

    pub async fn read_profile(&self, path: &PathBuf) -> Result<Profile> {
        let reader = Self::read_file(path).await?;
        let profile: Profile =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseModel(
                    "profile".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(profile)
    }

    pub async fn read_component_definition(
        &self,
        path: &PathBuf,
    ) -> Result<ComponentDefinition> {
        let reader = Self::read_file(path).await?;
        let component_definition: ComponentDefinition =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseModel(
                    "component_definition".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(component_definition)
    }

    pub async fn read_ssp(&self, path: &PathBuf) -> Result<SystemSecurityPlan> {
        let reader = Self::read_file(path).await?;
        let ssp: SystemSecurityPlan =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseModel(
                    "ssp".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(ssp)
    }

    pub async fn gen_models(&self, ctx: &DissectCtx) -> Result<()> {
        let model_loc = &ctx.model_loc;
        gen_dissect_dir(ctx).await?;

        match self {
            Self::AssessmentPlan => {
                let assessment_plan =
                    self.read_assessment_plan(model_loc).await?;

                AssessmentPlanBlock::gen_files(assessment_plan, ctx).await?;

                Ok(())
            }
            Self::AssessmentResults => {
                let assessment_results =
                    self.read_assessment_results(model_loc).await?;

                AssessmentResultsBlock::gen_files(assessment_results, ctx)
                    .await?;

                Ok(())
            }
            Self::Poam => {
                let poam = self.read_poam(model_loc).await?;

                PoamBlock::gen_files(poam, ctx).await?;

                Ok(())
            }
            Self::Catalog => {
                let catalog = self.read_catalog(model_loc).await?;

                CatalogBlock::gen_files(catalog, ctx).await?;

                Ok(())
            }
            Self::Profile => {
                let profile = self.read_profile(model_loc).await?;

                ProfileBlock::gen_files(profile, ctx).await?;

                Ok(())
            }
            Self::ComponentDefinition => {
                let component_definition =
                    self.read_component_definition(model_loc).await?;

                ComponentDefinitionBlock::gen_files(component_definition, ctx)
                    .await?;

                Ok(())
            }
            Self::Ssp => {
                let ssp = self.read_ssp(model_loc).await?;

                SspBlock::gen_files(ssp, ctx).await?;

                Ok(())
            }
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum AssessmentPlanBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "import_ssp")]
    ImportSsp,
    #[strum(serialize = "local_definitions")]
    LocalDefinitions,
    #[strum(serialize = "terms_and_conditions")]
    TermsAndConditions,
    #[strum(serialize = "reviewed_controls")]
    ReviewedControls,
    #[strum(serialize = "assessment_subjects")]
    AssessmentSubjects,
    #[strum(serialize = "assessment_assets")]
    AssessmentAssets,
    #[strum(serialize = "tasks")]
    Tasks,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl AssessmentPlanBlock {
    async fn gen_uuid(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid =
            serde_yaml::to_string(&assessment_plan.assessment_plan.uuid)?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "assessment plan".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata =
            serde_yaml::to_string(&assessment_plan.assessment_plan.metadata)?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_plan::DocumentMetadata> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "metadata".to_owned(),
                "assessment plan".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_import_ssp(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut import_ssp_yaml =
            File::create(format!("{}/import_ssp.yaml", path)).with_context(
                || {
                    format!(
                "Could not create import_ssp block at this location: {}",
                path
            )
                },
            )?;

        let import_ssp =
            serde_yaml::to_string(&assessment_plan.assessment_plan.import_ssp)?;

        import_ssp_yaml
            .write_all(import_ssp.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_import_ssp(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_plan::ImportSystemSecurityPlan>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let import_ssp = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "import_ssp".to_owned(),
                "assessment plan".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(import_ssp)
    }

    async fn gen_local_definitions(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut local_definitions_yaml =
            File::create(format!("{}/local_definitions.yaml", path))
                .with_context(|| {
                    format!(
                "Could not create local_definitions block at this location: {}",
                path
            )
                })?;

        if assessment_plan.assessment_plan.local_definitions.is_some() {
            let local_definitions = serde_yaml::to_string(
                &assessment_plan.assessment_plan.local_definitions,
            )?;

            local_definitions_yaml
                .write_all(local_definitions.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_local_definitions(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_plan::LocalDefinitions> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let local_definitions =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "local_definition".to_owned(),
                    "assessment plan".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(local_definitions)
    }

    async fn gen_terms_and_conditions(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut terms_and_conditions_yaml =
            File::create(format!("{}/terms_and_conditions.yaml", path))
                .with_context(|| {
                    format!(
                        "Could not create terms_and_conditions block at this location: {}",
                        path
                    )
                })?;

        if assessment_plan
            .assessment_plan
            .terms_and_conditions
            .is_some()
        {
            let terms_and_conditions = serde_yaml::to_string(
                &assessment_plan.assessment_plan.terms_and_conditions,
            )?;

            terms_and_conditions_yaml
                .write_all(terms_and_conditions.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_terms_and_conditions(
        &self,
        path: &PathBuf,
    ) -> Result<
    roscal_lib::assessment::assessment_plan::AssessmentPlanTermsAndConditions,
    >{
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let terms_and_conditions =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "terms_and_conditions".to_owned(),
                    "assessment plan".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(terms_and_conditions)
    }

    async fn gen_reviewed_controls(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut reviewed_controls_yaml =
            File::create(format!("{}/reviewed_controls.yaml", path))
                .with_context(|| {
                    format!(
                "Could not create reviewed_controls block at this location: {}",
                path
            )
                })?;

        let reviewed_controls = serde_yaml::to_string(
            &assessment_plan.assessment_plan.reviewed_controls,
        )?;

        reviewed_controls_yaml
            .write_all(reviewed_controls.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_reviewed_controls(&self, path: &PathBuf) -> Result<roscal_lib::assessment::assessment_plan::ReviewedControlsAndControlObjectives>{
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let reviewed_controls =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "reviewed_controls".to_owned(),
                    "assessment plan".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(reviewed_controls)
    }

    async fn gen_assessment_subjects(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut assessment_subjects_yaml =
            File::create(format!("{}/assessment_subjects.yaml", path))
                .with_context(|| {
                    format!(
                        "Could not create assessment_subjects block at this location: {}",
                        path
                    )
                })?;

        if assessment_plan
            .assessment_plan
            .assessment_subjects
            .is_some()
        {
            let assessment_subjects = serde_yaml::to_string(
                &assessment_plan.assessment_plan.assessment_subjects,
            )?;

            assessment_subjects_yaml
                .write_all(assessment_subjects.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_assessment_subjects(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::assessment_plan::SubjectOfAssessment>>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let assessment_subjects =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "assessment_subjects".to_owned(),
                    "assessment plan".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(assessment_subjects)
    }

    async fn gen_assessment_assets(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut assessment_assets_yaml =
            File::create(format!("{}/assessment_assets.yaml", path))
                .with_context(|| {
                    format!(
                "Could not create assessment_assets block at this location: {}",
                path
            )
                })?;

        if assessment_plan.assessment_plan.assessment_assets.is_some() {
            let assessment_assets = serde_yaml::to_string(
                &assessment_plan.assessment_plan.assessment_assets,
            )?;

            assessment_assets_yaml
                .write_all(assessment_assets.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_assessment_assets(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_plan::AssessmentAssets> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let assessment_assets =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "assessment_assets".to_owned(),
                    "assessment plan".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(assessment_assets)
    }

    async fn gen_tasks(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut tasks_yaml = File::create(format!("{}/tasks.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create tasks block at this location: {}",
                    path
                )
            })?;

        if assessment_plan.assessment_plan.tasks.is_some() {
            let tasks =
                serde_yaml::to_string(&assessment_plan.assessment_plan.tasks)?;

            tasks_yaml
                .write_all(tasks.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_tasks(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::assessment_plan::Task>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let tasks = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "tasks".to_owned(),
                "assessment plan".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(tasks)
    }

    async fn gen_back_matter(
        assessment_plan: &AssessmentPlan,
        path: &String,
    ) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if assessment_plan.assessment_plan.back_matter.is_some() {
            let back_matter = serde_yaml::to_string(
                &assessment_plan.assessment_plan.back_matter,
            )?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_plan::BackMatter> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "assessment plan".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(
        assessment_plan: AssessmentPlan,
        ctx: &DissectCtx,
    ) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided Assessment Plan block"
                    .to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&assessment_plan, modifiable).await?;
                    Self::gen_metadata(&assessment_plan, modifiable).await?;
                    Self::gen_import_ssp(&assessment_plan, modifiable).await?;
                    Self::gen_local_definitions(&assessment_plan, modifiable)
                        .await?;
                    Self::gen_terms_and_conditions(
                        &assessment_plan,
                        modifiable,
                    )
                    .await?;
                    Self::gen_reviewed_controls(&assessment_plan, modifiable)
                        .await?;
                    Self::gen_assessment_subjects(&assessment_plan, modifiable)
                        .await?;
                    Self::gen_assessment_assets(&assessment_plan, modifiable)
                        .await?;
                    Self::gen_tasks(&assessment_plan, modifiable).await?;
                    Self::gen_back_matter(&assessment_plan, modifiable).await?;
                }
                Self::Uuid => {
                    Self::gen_uuid(&assessment_plan, modifiable).await?
                }
                Self::Metadata => {
                    Self::gen_metadata(&assessment_plan, modifiable).await?
                }
                Self::ImportSsp => {
                    Self::gen_import_ssp(&assessment_plan, modifiable).await?
                }
                Self::LocalDefinitions => {
                    Self::gen_local_definitions(&assessment_plan, modifiable)
                        .await?
                }
                Self::TermsAndConditions => {
                    Self::gen_terms_and_conditions(&assessment_plan, modifiable)
                        .await?
                }
                Self::ReviewedControls => {
                    Self::gen_reviewed_controls(&assessment_plan, modifiable)
                        .await?
                }
                Self::AssessmentSubjects => {
                    Self::gen_assessment_subjects(&assessment_plan, modifiable)
                        .await?
                }
                Self::AssessmentAssets => {
                    Self::gen_assessment_assets(&assessment_plan, modifiable)
                        .await?
                }
                Self::Tasks => {
                    Self::gen_tasks(&assessment_plan, modifiable).await?
                }
                Self::BackMatter => {
                    Self::gen_back_matter(&assessment_plan, modifiable).await?
                }
            }
        }

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum AssessmentResultsBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "import_ap")]
    ImportAp,
    #[strum(serialize = "local_definitions")]
    LocalDefinitions,
    #[strum(serialize = "results")]
    Results,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl AssessmentResultsBlock {
    async fn gen_uuid(
        assessment_results: &AssessmentResults,
        path: &String,
    ) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid =
            serde_yaml::to_string(&assessment_results.assessment_results.uuid)?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "assessment results".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(
        assessment_results: &AssessmentResults,
        path: &String,
    ) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata = serde_yaml::to_string(
            &assessment_results.assessment_results.metadata,
        )?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_results::DocumentMetadata>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "metadata".to_owned(),
                "assessment results".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_import_ap(
        assessment_results: &AssessmentResults,
        path: &String,
    ) -> Result<()> {
        let mut import_ap_yaml =
            File::create(format!("{}/import_ap.yaml", path)).with_context(
                || {
                    format!(
                        "Could not create import_ap block at this location: {}",
                        path
                    )
                },
            )?;

        let import_ap = serde_yaml::to_string(
            &assessment_results.assessment_results.import_ap,
        )?;

        import_ap_yaml
            .write_all(import_ap.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_import_ap(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_results::ImportAssessmentPlan>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let import_ap = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "import_ap".to_owned(),
                "assessment results".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(import_ap)
    }

    async fn gen_local_definitions(
        assessment_results: &AssessmentResults,
        path: &String,
    ) -> Result<()> {
        let mut local_definitions_yaml =
            File::create(format!("{}/local_definitions.yaml", path))
                .with_context(|| {
                    format!(
                "Could not create local_definitions block at this location: {}",
                path
            )
                })?;

        if assessment_results
            .assessment_results
            .local_definitions
            .is_some()
        {
            let local_definitions = serde_yaml::to_string(
                &assessment_results.assessment_results.local_definitions,
            )?;

            local_definitions_yaml
                .write_all(local_definitions.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_local_definitions(&self, path: &PathBuf) -> Result<roscal_lib::assessment::assessment_results::AssessmentResultsLocalDefinitions>{
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let local_definitions =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "local_definition".to_owned(),
                    "assessment results".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(local_definitions)
    }

    async fn gen_results(
        assessment_results: &AssessmentResults,
        path: &String,
    ) -> Result<()> {
        let mut results_yaml = File::create(format!("{}/results.yaml", path))
            .with_context(|| {
            format!("Could not create results block at this location: {}", path)
        })?;

        let results = serde_yaml::to_string(
            &assessment_results.assessment_results.results,
        )?;

        results_yaml
            .write_all(results.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_results(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::assessment_results::AssessmentResult>>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let results = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "results".to_owned(),
                "assessment results".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(results)
    }

    async fn gen_back_matter(
        assessment_results: &AssessmentResults,
        path: &String,
    ) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if assessment_results.assessment_results.back_matter.is_some() {
            let back_matter = serde_yaml::to_string(
                &assessment_results.assessment_results.back_matter,
            )?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::assessment_results::BackMatter> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "assessment results".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(
        assessment_results: AssessmentResults,
        ctx: &DissectCtx,
    ) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided Assessment Result block"
                    .to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&assessment_results, modifiable).await?;
                    Self::gen_metadata(&assessment_results, modifiable).await?;
                    Self::gen_import_ap(&assessment_results, modifiable)
                        .await?;
                    Self::gen_local_definitions(
                        &assessment_results,
                        modifiable,
                    )
                    .await?;
                    Self::gen_results(&assessment_results, modifiable).await?;
                    Self::gen_back_matter(&assessment_results, modifiable)
                        .await?;
                }
                Self::Uuid => {
                    Self::gen_uuid(&assessment_results, modifiable).await?
                }
                Self::Metadata => {
                    Self::gen_metadata(&assessment_results, modifiable).await?
                }
                Self::ImportAp => {
                    Self::gen_import_ap(&assessment_results, modifiable).await?
                }
                Self::LocalDefinitions => {
                    Self::gen_local_definitions(&assessment_results, modifiable)
                        .await?
                }
                Self::Results => {
                    Self::gen_results(&assessment_results, modifiable).await?
                }
                Self::BackMatter => {
                    Self::gen_back_matter(&assessment_results, modifiable)
                        .await?
                }
            }
        }

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum PoamBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "import_ssp")]
    ImportSsp,
    #[strum(serialize = "system_id")]
    SystemId,
    #[strum(serialize = "local_definitions")]
    LocalDefinitions,
    #[strum(serialize = "observations")]
    Observations,
    #[strum(serialize = "risks")]
    Risks,
    #[strum(serialize = "findings")]
    Findings,
    #[strum(serialize = "poam_items")]
    PoamItems,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl PoamBlock {
    async fn gen_uuid(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid =
            serde_yaml::to_string(&poam.plan_of_action_and_milestones.uuid)?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata = serde_yaml::to_string(
            &poam.plan_of_action_and_milestones.metadata,
        )?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::poam::DocumentMetadata> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "metadata".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_import_ssp(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut import_ssp_yaml =
            File::create(format!("{}/import_ssp.yaml", path)).with_context(
                || {
                    format!(
                "Could not create import_ssp block at this location: {}",
                path
            )
                },
            )?;

        if poam.plan_of_action_and_milestones.import_ssp.is_some() {
            let import_ssp = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.import_ssp,
            )?;

            import_ssp_yaml
                .write_all(import_ssp.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_import_ssp(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::poam::ImportSystemSecurityPlan> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let import_ssp = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "import_ssp".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(import_ssp)
    }

    async fn gen_system_id(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut system_id_yaml =
            File::create(format!("{}/system_id.yaml", path)).with_context(
                || {
                    format!(
                        "Could not create system_id block at this location: {}",
                        path
                    )
                },
            )?;

        if poam.plan_of_action_and_milestones.system_id.is_some() {
            let system_id = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.system_id,
            )?;

            system_id_yaml
                .write_all(system_id.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_system_id(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::poam::SystemIdentification> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let system_id = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "system_id".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(system_id)
    }

    async fn gen_system_local_definitions(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut local_definitions_yaml =
            File::create(format!("{}/local_definitions.yaml", path))
                .with_context(|| {
                    format!(
                "Could not create local_definitions block at this location: {}",
                path
            )
                })?;

        if poam
            .plan_of_action_and_milestones
            .local_definitions
            .is_some()
        {
            let local_definitions = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.local_definitions,
            )?;

            local_definitions_yaml
                .write_all(local_definitions.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_system_local_definitions(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::poam::LocalDefinitions> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let system_local_definitions = serde_yaml::from_reader(reader)
            .map_err(|e| {
                CliError::ParseBlock(
                    "system_local_definition".to_owned(),
                    "poam".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(system_local_definitions)
    }

    async fn gen_observations(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut observations_yaml =
            File::create(format!("{}/observations.yaml", path)).with_context(
                || {
                    format!(
                "Could not create observations block at this location: {}",
                path
            )
                },
            )?;

        if poam.plan_of_action_and_milestones.observations.is_some() {
            let observations = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.observations,
            )?;

            observations_yaml
                .write_all(observations.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_observations(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::poam::Observation>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let observations = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "observation".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(observations)
    }

    async fn gen_risks(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut risks_yaml = File::create(format!("{}/risks.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create risks block at this location: {}",
                    path
                )
            })?;

        if poam.plan_of_action_and_milestones.risks.is_some() {
            let risks = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.risks,
            )?;

            risks_yaml
                .write_all(risks.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_risks(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::poam::IdentifiedRisk>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let risks = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "risks".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(risks)
    }

    async fn gen_findings(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut findings_yaml = File::create(format!("{}/findings.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create findings block at this location: {}",
                    path
                )
            })?;

        if poam.plan_of_action_and_milestones.findings.is_some() {
            let findings = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.findings,
            )?;

            findings_yaml
                .write_all(findings.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_findings(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::poam::Finding>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let findings = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "findings".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(findings)
    }

    async fn gen_poam_items(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut poam_items_yaml =
            File::create(format!("{}/poam_items.yaml", path)).with_context(
                || {
                    format!(
                "Could not create poam_items block at this location: {}",
                path
            )
                },
            )?;

        let poam_items = serde_yaml::to_string(
            &poam.plan_of_action_and_milestones.poam_items,
        )?;

        poam_items_yaml
            .write_all(poam_items.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_poam_items(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::assessment::poam::PoaMItem>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let poam_items = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "poam_item".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(poam_items)
    }

    async fn gen_back_matter(
        poam: &PlanOfActionAndMilestones,
        path: &String,
    ) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if poam.plan_of_action_and_milestones.back_matter.is_some() {
            let back_matter = serde_yaml::to_string(
                &poam.plan_of_action_and_milestones.back_matter,
            )?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::assessment::poam::BackMatter> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "poam".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(
        poam: PlanOfActionAndMilestones,
        ctx: &DissectCtx,
    ) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided Poam block".to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&poam, modifiable).await?;
                    Self::gen_metadata(&poam, modifiable).await?;
                    Self::gen_import_ssp(&poam, modifiable).await?;
                    Self::gen_system_id(&poam, modifiable).await?;
                    Self::gen_system_local_definitions(&poam, modifiable)
                        .await?;
                    Self::gen_observations(&poam, modifiable).await?;
                    Self::gen_risks(&poam, modifiable).await?;
                    Self::gen_findings(&poam, modifiable).await?;
                    Self::gen_poam_items(&poam, modifiable).await?;
                    Self::gen_back_matter(&poam, modifiable).await?;
                }
                Self::Uuid => Self::gen_uuid(&poam, modifiable).await?,
                Self::Metadata => Self::gen_metadata(&poam, modifiable).await?,
                Self::ImportSsp => {
                    Self::gen_import_ssp(&poam, modifiable).await?
                }
                Self::SystemId => {
                    Self::gen_system_id(&poam, modifiable).await?
                }
                Self::LocalDefinitions => {
                    Self::gen_system_local_definitions(&poam, modifiable)
                        .await?
                }
                Self::Observations => {
                    Self::gen_observations(&poam, modifiable).await?
                }
                Self::Risks => Self::gen_risks(&poam, modifiable).await?,
                Self::Findings => Self::gen_findings(&poam, modifiable).await?,
                Self::PoamItems => {
                    Self::gen_poam_items(&poam, modifiable).await?
                }
                Self::BackMatter => {
                    Self::gen_back_matter(&poam, modifiable).await?
                }
            }
        }

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum CatalogBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "params")]
    Params,
    #[strum(serialize = "controls")]
    Controls,
    #[strum(serialize = "groups")]
    Groups,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl CatalogBlock {
    async fn gen_uuid(catalog: &Catalog, path: &String) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid = serde_yaml::to_string(&catalog.catalog.uuid)?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "catalog".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(catalog: &Catalog, path: &String) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata = serde_yaml::to_string(&catalog.catalog.metadata)?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::control::catalog::DocumentMetadata> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "metadata".to_owned(),
                "catalog".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_params(catalog: &Catalog, path: &String) -> Result<()> {
        let mut params_yaml = File::create(format!("{}/params.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create params block at this location: {}",
                    path
                )
            })?;

        if catalog.catalog.params.is_some() {
            let params = serde_yaml::to_string(&catalog.catalog.params)?;

            params_yaml
                .write_all(params.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_params(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::control::catalog::Parameter>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let params = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "params".to_owned(),
                "catalog".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(params)
    }

    async fn gen_controls(catalog: &Catalog, path: &String) -> Result<()> {
        let mut controls_yaml = File::create(format!("{}/controls.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create controls block at this location: {}",
                    path
                )
            })?;

        if catalog.catalog.controls.is_some() {
            let controls = serde_yaml::to_string(&catalog.catalog.controls)?;

            controls_yaml
                .write_all(controls.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_controls(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::control::catalog::Control>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let controls = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "controls".to_owned(),
                "catalog".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        println!("{:?}", controls);
        Ok(controls)
    }

    async fn gen_groups(catalog: &Catalog, path: &String) -> Result<()> {
        let mut groups_yaml = File::create(format!("{}/groups.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create groups block at this location: {}",
                    path
                )
            })?;

        if catalog.catalog.groups.is_some() {
            let groups = serde_yaml::to_string(&catalog.catalog.groups)?;

            groups_yaml
                .write_all(groups.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_groups(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::control::catalog::ControlGroup>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let groups = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "groups".to_owned(),
                "catalog".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(groups)
    }

    async fn gen_back_matter(catalog: &Catalog, path: &String) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if catalog.catalog.back_matter.is_some() {
            let back_matter =
                serde_yaml::to_string(&catalog.catalog.back_matter)
                    .with_context(|| {
                        "Could not convert to YAML file".to_owned()
                    })?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::control::catalog::BackMatter> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "catalog".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(catalog: Catalog, ctx: &DissectCtx) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided Catalog block".to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&catalog, modifiable).await?;
                    Self::gen_metadata(&catalog, modifiable).await?;
                    Self::gen_params(&catalog, modifiable).await?;
                    Self::gen_controls(&catalog, modifiable).await?;
                    Self::gen_groups(&catalog, modifiable).await?;
                    Self::gen_back_matter(&catalog, modifiable).await?;
                }
                Self::Uuid => Self::gen_uuid(&catalog, modifiable).await?,
                Self::Metadata => {
                    Self::gen_metadata(&catalog, modifiable).await?
                }
                Self::Params => Self::gen_params(&catalog, modifiable).await?,
                Self::Controls => {
                    Self::gen_controls(&catalog, modifiable).await?
                }
                Self::Groups => Self::gen_groups(&catalog, modifiable).await?,
                Self::BackMatter => {
                    Self::gen_back_matter(&catalog, modifiable).await?
                }
            }
        }

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum ProfileBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "imports")]
    Imports,
    #[strum(serialize = "merge")]
    Merge,
    #[strum(serialize = "modify")]
    Modify,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl ProfileBlock {
    async fn gen_uuid(profile: &Profile, path: &String) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid = serde_yaml::to_string(&profile.profile.uuid)?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "profile".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(profile: &Profile, path: &String) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata = serde_yaml::to_string(&profile.profile.metadata)?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::control::profile::DocumentMetadata> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "metadata".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_imports(profile: &Profile, path: &String) -> Result<()> {
        let mut imports_yaml = File::create(format!("{}/imports.yaml", path))
            .with_context(|| {
            format!("Could not create imports block at this location: {}", path)
        })?;

        let imports = serde_yaml::to_string(&profile.profile.imports)?;

        imports_yaml
            .write_all(imports.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_imports(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::control::profile::ImportResource>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let imports = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "imports".to_owned(),
                "profile".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(imports)
    }

    async fn gen_merge(profile: &Profile, path: &String) -> Result<()> {
        let mut merge_yaml = File::create(format!("{}/merge.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create merge block at this location: {}",
                    path
                )
            })?;

        if profile.profile.merge.is_some() {
            let merge = serde_yaml::to_string(&profile.profile.merge)?;

            merge_yaml
                .write_all(merge.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_merge(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::control::profile::MergeControls> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let merge = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "merge".to_owned(),
                "profile".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(merge)
    }

    async fn gen_modify(profile: &Profile, path: &String) -> Result<()> {
        let mut modify_yaml = File::create(format!("{}/modify.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create modify block at this location: {}",
                    path
                )
            })?;

        if profile.profile.modify.is_some() {
            let modify = serde_yaml::to_string(&profile.profile.modify)?;

            modify_yaml
                .write_all(modify.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_modify(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::control::profile::ModifyControls> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let modify = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "modify".to_owned(),
                "profile".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(modify)
    }

    async fn gen_back_matter(profile: &Profile, path: &String) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if profile.profile.back_matter.is_some() {
            let back_matter =
                serde_yaml::to_string(&profile.profile.back_matter)
                    .with_context(|| {
                        "Could not convert to YAML file".to_owned()
                    })?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::control::profile::BackMatter> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "profile".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(profile: Profile, ctx: &DissectCtx) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided Profile block".to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&profile, modifiable).await?;
                    Self::gen_metadata(&profile, modifiable).await?;
                    Self::gen_imports(&profile, modifiable).await?;
                    Self::gen_merge(&profile, modifiable).await?;
                    Self::gen_modify(&profile, modifiable).await?;
                    Self::gen_back_matter(&profile, modifiable).await?;
                }
                Self::Uuid => Self::gen_uuid(&profile, modifiable).await?,
                Self::Metadata => {
                    Self::gen_metadata(&profile, modifiable).await?
                }
                Self::Imports => {
                    Self::gen_imports(&profile, modifiable).await?
                }
                Self::Merge => Self::gen_merge(&profile, modifiable).await?,
                Self::Modify => Self::gen_modify(&profile, modifiable).await?,
                Self::BackMatter => {
                    Self::gen_back_matter(&profile, modifiable).await?
                }
            }
        }

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum ComponentDefinitionBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "import_component_definitions")]
    ImportComponentDefinitions,
    #[strum(serialize = "components")]
    Components,
    #[strum(serialize = "capabilities")]
    Capabilities,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl ComponentDefinitionBlock {
    async fn gen_uuid(
        component_definition: &ComponentDefinition,
        path: &String,
    ) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid = serde_yaml::to_string(
            &component_definition.component_definition.uuid,
        )?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "component definition".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(
        component_definition: &ComponentDefinition,
        path: &String,
    ) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata = serde_yaml::to_string(
            &component_definition.component_definition.metadata,
        )?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<
        roscal_lib::implementation::component_definition::DocumentMetadata,
    > {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "metadata".to_owned(),
                "component definition".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_import_component_definitions(
        component_definition: &ComponentDefinition,
        path: &String,
    ) -> Result<()> {
        let mut import_component_definitions_yaml =
            File::create(format!("{}/import_component_definitions.yaml", path))
                .with_context(|| {
                    format!(
                        "Could not create import_component_definitions block at this location: {}",
                        path
                    )
                })?;

        if component_definition
            .component_definition
            .import_component_definitions
            .is_some()
        {
            let import_component_definitions = serde_yaml::to_string(
                &component_definition
                    .component_definition
                    .import_component_definitions,
            )?;

            import_component_definitions_yaml
                .write_all(import_component_definitions.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_import_component_definitions(&self, path: &PathBuf) -> Result<Vec<roscal_lib::implementation::component_definition::ImportComponentDefinition>>{
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let import_component_definitions = serde_yaml::from_reader(reader)
            .map_err(|e| {
                CliError::ParseBlock(
                    "import_component_definitions".to_owned(),
                    "component definition".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(import_component_definitions)
    }

    async fn gen_components(
        component_definition: &ComponentDefinition,
        path: &String,
    ) -> Result<()> {
        let mut components_yaml =
            File::create(format!("{}/components.yaml", path)).with_context(
                || {
                    format!(
                "Could not create components block at this location: {}",
                path
            )
                },
            )?;

        if component_definition
            .component_definition
            .components
            .is_some()
        {
            let components = serde_yaml::to_string(
                &component_definition.component_definition.components,
            )?;

            components_yaml
                .write_all(components.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_components(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::implementation::component_definition::Component>>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let components = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "component".to_owned(),
                "component definition".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(components)
    }

    async fn gen_capabilities(
        component_definition: &ComponentDefinition,
        path: &String,
    ) -> Result<()> {
        let mut capabilities_yaml =
            File::create(format!("{}/capabilities.yaml", path)).with_context(
                || {
                    format!(
                "Could not create capabilities block at this location: {}",
                path
            )
                },
            )?;

        if component_definition
            .component_definition
            .capabilities
            .is_some()
        {
            let capabilities = serde_yaml::to_string(
                &component_definition.component_definition.capabilities,
            )?;

            capabilities_yaml
                .write_all(capabilities.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_capabilities(
        &self,
        path: &PathBuf,
    ) -> Result<Vec<roscal_lib::implementation::component_definition::Capability>>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let capabilities = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "capabilities".to_owned(),
                "component definition".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(capabilities)
    }

    async fn gen_back_matter(
        component_definition: &ComponentDefinition,
        path: &String,
    ) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if component_definition
            .component_definition
            .back_matter
            .is_some()
        {
            let back_matter = serde_yaml::to_string(
                &component_definition.component_definition.back_matter,
            )?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::component_definition::BackMatter>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "component definition".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(
        component_definition: ComponentDefinition,
        ctx: &DissectCtx,
    ) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided Component Definition block".to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&component_definition, modifiable).await?;
                    Self::gen_metadata(&component_definition, modifiable)
                        .await?;
                    Self::gen_import_component_definitions(
                        &component_definition,
                        modifiable,
                    )
                    .await?;
                    Self::gen_components(&component_definition, modifiable)
                        .await?;
                    Self::gen_components(&component_definition, modifiable)
                        .await?;
                    Self::gen_capabilities(&component_definition, modifiable)
                        .await?;
                    Self::gen_back_matter(&component_definition, modifiable)
                        .await?;
                }
                Self::Uuid => {
                    Self::gen_uuid(&component_definition, modifiable).await?
                }
                Self::Metadata => {
                    Self::gen_metadata(&component_definition, modifiable)
                        .await?
                }
                Self::ImportComponentDefinitions => {
                    Self::gen_import_component_definitions(
                        &component_definition,
                        modifiable,
                    )
                    .await?
                }
                Self::Components => {
                    Self::gen_components(&component_definition, modifiable)
                        .await?
                }
                Self::Capabilities => {
                    Self::gen_capabilities(&component_definition, modifiable)
                        .await?
                }
                Self::BackMatter => {
                    Self::gen_back_matter(&component_definition, modifiable)
                        .await?
                }
            }
        }

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub(super) enum SspBlock {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "uuid")]
    Uuid,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "import_profile")]
    ImportProfile,
    #[strum(serialize = "system_characteristics")]
    SystemCharacteristics,
    #[strum(serialize = "system_implementation")]
    SystemImplementation,
    #[strum(serialize = "control_implementation")]
    ControlImplementation,
    #[strum(serialize = "back_matter")]
    BackMatter,
}

impl SspBlock {
    async fn gen_uuid(ssp: &SystemSecurityPlan, path: &String) -> Result<()> {
        let mut uuid_yaml = File::create(format!("{}/uuid.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create uuid block at this location: {}",
                    path
                )
            })?;

        let uuid = serde_yaml::to_string(&ssp.system_security_plan.uuid)?;

        uuid_yaml
            .write_all(uuid.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_uuid(&self, path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let uuid = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "uuid".to_owned(),
                "ssp".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(uuid)
    }

    async fn gen_metadata(
        ssp: &SystemSecurityPlan,
        path: &String,
    ) -> Result<()> {
        let mut metadata_yaml = File::create(format!("{}/metadata.yaml", path))
            .with_context(|| {
                format!(
                    "Could not create metadata block at this location: {}",
                    path
                )
            })?;

        let metadata =
            serde_yaml::to_string(&ssp.system_security_plan.metadata)?;

        metadata_yaml
            .write_all(metadata.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_metadata(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::ssp::DocumentMetadata> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "metadata".to_owned(),
                "ssp".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(metadata)
    }

    async fn gen_import_profile(
        ssp: &SystemSecurityPlan,
        path: &String,
    ) -> Result<()> {
        let mut import_profile_yaml =
            File::create(format!("{}/import_profile.yaml", path))
                .with_context(|| {
                    format!(
                "Could not create import_profile block at this location: {}",
                path
            )
                })?;

        let import_profile =
            serde_yaml::to_string(&ssp.system_security_plan.import_profile)?;

        import_profile_yaml
            .write_all(import_profile.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_import_profile(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::ssp::ImportProfile> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let import_profile = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "import_profile".to_owned(),
                "ssp".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(import_profile)
    }

    async fn gen_system_characteristics(
        ssp: &SystemSecurityPlan,
        path: &String,
    ) -> Result<()> {
        let mut system_characteristics_yaml =
            File::create(format!("{}/system_characteristics.yaml", path))
                .with_context(|| {
                    format!(
                        "Could not create system_characteristics block at this location: {}",
                        path
                    )
                })?;

        let system_characteristics = serde_yaml::to_string(
            &ssp.system_security_plan.system_characteristics,
        )?;

        system_characteristics_yaml
            .write_all(system_characteristics.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_system_characteristics(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::ssp::SystemCharacteristics> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let system_characteristics =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "system_characteristics".to_owned(),
                    "ssp".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(system_characteristics)
    }

    async fn gen_system_implementation(
        ssp: &SystemSecurityPlan,
        path: &String,
    ) -> Result<()> {
        let mut system_implementation_yaml =
            File::create(format!("{}/system_implementation.yaml", path))
                .with_context(|| {
                    format!(
                        "Could not create system_implementation block at this location: {}",
                        path
                    )
                })?;

        let system_implementation = serde_yaml::to_string(
            &ssp.system_security_plan.system_implementation,
        )?;

        system_implementation_yaml
            .write_all(system_implementation.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_system_implementation(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::ssp::SystemImplementation> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let system_implementation =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "system_implementation".to_owned(),
                    "ssp".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(system_implementation)
    }

    async fn gen_control_implementation(
        ssp: &SystemSecurityPlan,
        path: &String,
    ) -> Result<()> {
        let mut control_implementation_yaml =
            File::create(format!("{}/control_implementation.yaml", path))
                .with_context(|| {
                    format!(
                        "Could not create control_implementation block at this location: {}",
                        path
                    )
                })?;

        let control_implementation = serde_yaml::to_string(
            &ssp.system_security_plan.control_implementation,
        )?;

        control_implementation_yaml
            .write_all(control_implementation.as_bytes())
            .with_context(|| "Could not write to manifest".to_owned())?;

        Ok(())
    }

    pub(super) async fn read_control_implementation(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::ssp::ControlImplementation> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let control_implementation =
            serde_yaml::from_reader(reader).map_err(|e| {
                CliError::ParseBlock(
                    "control_implementation".to_owned(),
                    "ssp".to_owned(),
                    path.to_string_lossy().into_owned(),
                    e.to_string(),
                )
            })?;

        Ok(control_implementation)
    }

    async fn gen_back_matter(
        ssp: &SystemSecurityPlan,
        path: &String,
    ) -> Result<()> {
        let mut back_matter_yaml =
            File::create(format!("{}/back_matter.yaml", path)).with_context(
                || {
                    format!(
                "Could not create back_matter block at this location: {}",
                path
            )
                },
            )?;

        if ssp.system_security_plan.back_matter.is_some() {
            let back_matter =
                serde_yaml::to_string(&ssp.system_security_plan.back_matter)
                    .with_context(|| {
                        "Could not convert to YAML file".to_owned()
                    })?;

            back_matter_yaml
                .write_all(back_matter.as_bytes())
                .with_context(|| "Could not write to manifest".to_owned())?;
        }

        Ok(())
    }

    pub(super) async fn read_back_matter(
        &self,
        path: &PathBuf,
    ) -> Result<roscal_lib::implementation::ssp::BackMatter> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let back_matter = serde_yaml::from_reader(reader).map_err(|e| {
            CliError::ParseBlock(
                "back_matter".to_owned(),
                "ssp".to_owned(),
                path.to_string_lossy().into_owned(),
                e.to_string(),
            )
        })?;

        Ok(back_matter)
    }

    pub async fn gen_files(
        ssp: SystemSecurityPlan,
        ctx: &DissectCtx,
    ) -> Result<()> {
        let modifiable = &ctx.modifiable;

        for i in &ctx.blocks {
            let block = Self::from_str(i).with_context(|| {
                "Could not determine the provided System Security Plan block".to_string()
            })?;

            match block {
                Self::All => {
                    Self::gen_uuid(&ssp, modifiable).await?;
                    Self::gen_metadata(&ssp, modifiable).await?;
                    Self::gen_import_profile(&ssp, modifiable).await?;
                    Self::gen_system_characteristics(&ssp, modifiable).await?;
                    Self::gen_system_implementation(&ssp, modifiable).await?;
                    Self::gen_control_implementation(&ssp, modifiable).await?;
                    Self::gen_back_matter(&ssp, modifiable).await?;
                }
                Self::Uuid => Self::gen_uuid(&ssp, modifiable).await?,
                Self::Metadata => Self::gen_metadata(&ssp, modifiable).await?,
                Self::ImportProfile => {
                    Self::gen_import_profile(&ssp, modifiable).await?
                }
                Self::SystemCharacteristics => {
                    Self::gen_system_characteristics(&ssp, modifiable).await?
                }
                Self::SystemImplementation => {
                    Self::gen_system_implementation(&ssp, modifiable).await?
                }
                Self::ControlImplementation => {
                    Self::gen_control_implementation(&ssp, modifiable).await?
                }
                Self::BackMatter => {
                    Self::gen_back_matter(&ssp, modifiable).await?
                }
            }
        }

        Ok(())
    }
}
