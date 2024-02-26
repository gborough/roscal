use std::str::FromStr;

use super::model::*;

pub(super) fn is_valid_model(model: &str) -> bool {
    if OscalModels::from_str(model).is_err() {
        eprintln!("Invalid model provided: {}", model);
        return false;
    }

    true
}

pub(super) fn is_valid_assessment_plan_block(blocks: &Vec<String>) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| AssessmentPlanBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid assessment plan block provided: {:?}", blocks);
        false
    }
}

pub(super) fn is_valid_assessment_results_block(blocks: &Vec<String>) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| AssessmentResultsBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid assessment results block provided: {:?}", blocks);
        false
    }
}

pub(super) fn is_valid_poam_block(blocks: &Vec<String>) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| PoamBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid poam block provided: {:?}", blocks);
        false
    }
}

pub(super) fn is_valid_catalog_block(blocks: &Vec<String>) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| CatalogBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid catalog block provided: {:?}", blocks);
        false
    }
}

pub(super) fn is_valid_profile_block(blocks: &Vec<String>) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| ProfileBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid profile block provided: {:?}", blocks);
        false
    }
}

pub(super) fn is_valid_component_definition_block(
    blocks: &Vec<String>,
) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| ComponentDefinitionBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid component definition block provided: {:?}", blocks);
        false
    }
}

pub(super) fn is_valid_ssp_block(blocks: &Vec<String>) -> bool {
    let mut col = String::new();

    if blocks.len() > 1 && blocks.iter().any(|v| v == "all") {
        eprintln!("Multiple block cannot include `all` option");
        return false;
    }

    blocks
        .iter()
        .filter(|v| SspBlock::from_str(v).is_err())
        .for_each(|v| col.push_str(v));

    if col.is_empty() {
        true
    } else {
        eprintln!("Invalid ssp block provided: {:?}", blocks);
        false
    }
}
