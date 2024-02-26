use std::{fs::File, io::BufReader};

use roscal_lib::{
    control::profile::Profile, implementation::ssp::SystemSecurityPlan,
};

#[test]
fn wrong_date() {
    let file = File::open("tests/malformed/date.yaml").unwrap();
    let reader = BufReader::new(file);
    let res: Result<SystemSecurityPlan, serde_yaml::Error> =
        serde_yaml::from_reader(reader);

    let e = res.err().unwrap().to_string();

    assert_eq!("system-security-plan.system-characteristics: invalid date pattern at line 54 column 5", e)
}

#[test]
fn wrong_datetime() {
    let file = File::open("tests/malformed/datetime.yaml").unwrap();
    let reader = BufReader::new(file);
    let res: Result<Profile, serde_yaml::Error> =
        serde_yaml::from_reader(reader);

    let e = res.err().unwrap().to_string();

    assert_eq!(
        "profile.metadata: invalid datetime pattern at line 4 column 5",
        e
    )
}

#[test]
fn wrong_email() {
    let file = File::open("tests/malformed/email.yaml").unwrap();
    let reader = BufReader::new(file);
    let res: Result<Profile, serde_yaml::Error> =
        serde_yaml::from_reader(reader);

    let e = res.err().unwrap().to_string();

    assert_eq!("profile.metadata.parties[0]: invalid email pattern at line 14 column 9", e)
}

#[test]
fn wrong_uri() {
    let file = File::open("tests/malformed/uri.yaml").unwrap();
    let reader = BufReader::new(file);
    let res: Result<Profile, serde_yaml::Error> =
        serde_yaml::from_reader(reader);

    let e = res.err().unwrap().to_string();

    assert_eq!("profile.back-matter.resources[0].rlinks[0]: invalid uri reference pattern at line 416 column 13", e)
}

#[test]
fn wrong_uuid() {
    let file = File::open("tests/malformed/uuid.yaml").unwrap();
    let reader = BufReader::new(file);
    let res: Result<Profile, serde_yaml::Error> =
        serde_yaml::from_reader(reader);

    let e = res.err().unwrap().to_string();

    assert_eq!("profile: invalid uuid pattern at line 2 column 3", e)
}
