use std::{
    fs::{self, read_to_string, File},
    io::BufReader,
};

use assert_cmd::Command;
use roscal_lib::control::catalog::Catalog;
use tempfile::tempdir;
use walkdir::WalkDir;

#[test]
fn cli_merge_update_uuid() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/catalog.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Catalog",
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
        let assert_v4 = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
                "--update-uuid",
                "v4",
            ])
            .assert();
        assert_v4.success();

        let mut cmd = Command::cargo_bin("roscal").unwrap();
        let assert_v5 = cmd
            .args(&[
                "merge",
                "--dir",
                path,
                "--output-format",
                "yaml",
                "--output-dir",
                path,
                "--update-uuid",
                "v5",
            ])
            .assert();
        assert_v5.success();
    }
    dir.close().unwrap()
}

#[test]
fn cli_merge_modify_content() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/catalog.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );
    let uuid_dir = format!(
        "{}/tests/data/edit_uuid",
        std::env::current_dir().unwrap().to_string_lossy()
    );
    let back_matter_dir = format!(
        "{}/tests/data/edit_back_matter",
        std::env::current_dir().unwrap().to_string_lossy()
    );
    let uuid = read_to_string(uuid_dir).unwrap();
    let back_matter = read_to_string(back_matter_dir).unwrap();

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Catalog",
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

        fs::write(format!("{}/modifiable/uuid.yaml", path), &uuid).unwrap();
        fs::write(
            format!("{}/modifiable/back_matter.yaml", path),
            &back_matter,
        )
        .unwrap();

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

        let file = File::open(format!("{}/merged.yaml", path)).unwrap();
        let reader = BufReader::new(file);
        let res: Catalog = serde_yaml::from_reader(reader).unwrap();
        assert_eq!(
            "84cbf061-eb87-4ec1-8112-1f529232e907",
            res.catalog.back_matter.unwrap().resources.unwrap()[0].uuid
        );
        assert_eq!("74c8ba1e-5cd4-4ad1-bbfd-d888e2f6c721", res.catalog.uuid);
    }
    dir.close().unwrap()
}

#[test]
fn cli_merge_wrong_dir() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();

    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "merge",
            "--dir",
            "tests/data/not",
            "--output-format",
            "yaml",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.stderr("Invalid directory provided: tests/data/not\n");
    dir.close().unwrap()
}

#[test]
fn cli_merge_wrong_output_format() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/catalog.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Catalog",
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
                "ron",
                "--output-dir",
                path,
            ])
            .assert();
        assert.stderr("Error: Unknown output format. Use json or yaml\n");
    }
    dir.close().unwrap()
}

#[test]
fn cli_merge_wrong_uuid() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let dissect_dir = format!(
        "{}/tests/data/catalog.yaml",
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut cmd_dissect = Command::cargo_bin("roscal").unwrap();
    let assert_dissect = cmd_dissect
        .args(&[
            "dissect",
            "--file",
            &dissect_dir,
            "--model",
            "Catalog",
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
                "--update-uuid",
                "v8",
            ])
            .assert();
        assert.stderr("Error: Unknown uuid version. Use v4 or v5\n");
    }
    dir.close().unwrap()
}
