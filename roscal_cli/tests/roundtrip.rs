use assert_cmd::Command;
use tempfile::tempdir;
use walkdir::WalkDir;

#[test]
fn cli_roundtrip_ap() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/ap.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "AssessmentPlan",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert_dissect.success();

    for entry in WalkDir::new(dir_arg).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
            ])
            .assert();
        assert.success();
    }
    dir.close().unwrap()
}

#[test]
fn cli_roundtrip_ar() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/ar.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "AssessmentResults",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert_dissect.success();

    for entry in WalkDir::new(dir_arg).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
            ])
            .assert();
        assert.success();
    }
    dir.close().unwrap()
}

#[test]
fn cli_roundtrip_comp_def() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/compdef.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "ComponentDefinition",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert_dissect.success();

    for entry in WalkDir::new(dir_arg).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
            ])
            .assert();
        assert.success();
    }
    dir.close().unwrap()
}

#[test]
fn cli_roundtrip_poam() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/poam.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Poam",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert_dissect.success();

    for entry in WalkDir::new(dir_arg).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
            ])
            .assert();
        assert.success();
    }
    dir.close().unwrap()
}

#[test]
fn cli_roundtrip_profile() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/profile.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Profile",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert_dissect.success();

    for entry in WalkDir::new(dir_arg).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
            ])
            .assert();
        assert.success();
    }
    dir.close().unwrap()
}

#[test]
fn cli_roundtrip_ssp() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/ssp.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Ssp",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert_dissect.success();

    for entry in WalkDir::new(dir_arg).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
            ])
            .assert();
        assert.success();
    }
    dir.close().unwrap()
}
