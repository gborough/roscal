use std::{
    fs::{read_dir, File},
    io::BufReader,
};

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

#[test]
fn parse_json_ap() {
    for e in read_dir("tests/ap/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<AssessmentPlan, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_ap() {
    for e in read_dir("tests/ap/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<AssessmentPlan, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_json_ar() {
    for e in read_dir("tests/ar/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<AssessmentResults, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_ar() {
    for e in read_dir("tests/ar/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<AssessmentResults, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_json_catalog() {
    for e in read_dir("tests/catalog/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<Catalog, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_catalog() {
    for e in read_dir("tests/catalog/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<Catalog, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_json_component_def() {
    for e in read_dir("tests/component-definition/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<ComponentDefinition, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_component_def() {
    for e in read_dir("tests/component-definition/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<ComponentDefinition, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_json_poam() {
    for e in read_dir("tests/poam/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<PlanOfActionAndMilestones, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_poam() {
    for e in read_dir("tests/poam/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<PlanOfActionAndMilestones, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_json_profile() {
    for e in read_dir("tests/profile/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<Profile, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_profile() {
    for e in read_dir("tests/profile/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<Profile, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_json_ssp() {
    for e in read_dir("tests/ssp/json").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let res: Result<SystemSecurityPlan, serde_json::Error> =
            serde_json::from_reader(reader);
        assert_eq!(true, res.is_ok())
    }
}

#[test]
fn parse_yaml_ssp() {
    for e in read_dir("tests/ssp/yaml").unwrap() {
        let entry = e.unwrap();
        let path = entry.path();
        let file = File::open(path.clone()).unwrap();
        let reader = BufReader::new(file);
        let res: Result<SystemSecurityPlan, serde_yaml::Error> =
            serde_yaml::from_reader(reader);
        let a = res.map_err(|e| eprintln!("{}:::{}", e, path.display()));
        assert_eq!(true, a.is_ok())
    }
}
