use std::{fs::File, io::BufReader, str::FromStr};

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

use crate::cli::cli_opts::Validate;

use super::{model::OscalModels, validation::is_valid_model};

pub(super) async fn validate_model(opts: &Validate) -> Result<()> {
    if !is_valid_model(&opts.model) {
        std::process::exit(1)
    }

    let file = File::open(&opts.file).with_context(|| {
        format!("Could not open model file: `{}`", &opts.file.display())
    })?;

    let reader = BufReader::new(file);

    match OscalModels::from_str(&opts.model).with_context(|| {
        "Could not determine the provided OSCAL model".to_string()
    })? {
        OscalModels::AssessmentPlan => {
            let res: Result<AssessmentPlan, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid Assessment Plan model")
            } else {
                println!("This is not a valid Assessment Plan model")
            }
        }
        OscalModels::AssessmentResults => {
            let res: Result<AssessmentResults, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid Assessment Result model")
            } else {
                println!("This is not a valid Assessment Result model")
            }
        }
        OscalModels::Poam => {
            let res: Result<PlanOfActionAndMilestones, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid Plan of Action and Milestones model")
            } else {
                println!(
                    "This is not a valid Plan of Action and Milestones model"
                )
            }
        }
        OscalModels::Catalog => {
            let res: Result<Catalog, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid Catalog model")
            } else {
                println!("This is not a valid Catalog model")
            }
        }
        OscalModels::Profile => {
            let res: Result<Profile, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid Profile model")
            } else {
                println!("This is not a valid Profile model")
            }
        }
        OscalModels::ComponentDefinition => {
            let res: Result<ComponentDefinition, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid Component Definition model")
            } else {
                println!("This is not a valid Component Definition model")
            }
        }
        OscalModels::Ssp => {
            let res: Result<SystemSecurityPlan, serde_json::Error> =
                serde_json::from_reader(reader);
            if res.is_ok() {
                println!("This is a valid System Security Plan model")
            } else {
                println!("This is not a valid System Security Plan model")
            }
        }
    }

    Ok(())
}
