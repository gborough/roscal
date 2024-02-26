use crate::models::workspace::{Validator, Workspace};

use super::cli_opts::{Dissect, Merge, Validate};

pub async fn run_dissect(
    opts: &Dissect,
) -> Result<(), Box<dyn std::error::Error>> {
    Workspace::dissect(opts).await?;

    Ok(())
}

pub async fn run_merge(opts: &Merge) -> Result<(), Box<dyn std::error::Error>> {
    Workspace::merge(opts).await?;

    Ok(())
}

pub async fn run_validate(
    opts: &Validate,
) -> Result<(), Box<dyn std::error::Error>> {
    Validator::validate(opts).await?;

    Ok(())
}

pub async fn show_dissect() -> Result<(), Box<dyn std::error::Error>> {
    let desc = r#"
Available Model Options:

  One of:
    - AssessmentPlan
    - AssessmentResults
    - Poam
    - Catalog
    - Profile
    - ComponentDefinition
    - Ssp

Available Blocks Options:

  Under Assessment Plan Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - import_ssp
      - local_definitions
      - terms_and_conditions
      - reviewed_controls
      - assessment_subjects
      - assessment_assets
      - tasks
      - back_matter

  Under Assessment Results Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - import_ap
      - local_definitions
      - results
      - back_matter

  Under Plan of Action and Milestones Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - import_ssp
      - system_id
      - local_definitions
      - observations
      - risks
      - findings
      - poam_items
      - back_matter

  Under Catalog Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - params
      - controls
      - groups
      - back_matter

  Under Profile Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - imports
      - merge
      - modify
      - back_matter

  Under Component Definition Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - import_component_definitions
      - components
      - capabilities
      - back_matter

  Under System Security Plan Model:

    One of:
      - all

    Or

    Any of:
      - uuid
      - metadata
      - import_profile
      - system_characteristics
      - system_implementation
      - control_implementation
      - back_matter
        "#;

    println!("{desc}");

    Ok(())
}
