use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn cli_dissect_single_block() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "Catalog",
            "--blocks",
            "uuid",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.success();
    dir.close().unwrap()
}

#[test]
fn cli_dissect_all_block() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "Catalog",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.success();
    dir.close().unwrap()
}

#[test]
fn cli_dissect_multiple_blocks() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "Catalog",
            "--blocks",
            "uuid,metadata",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.success();
    dir.close().unwrap()
}

#[test]
fn cli_dissect_multiple_dup_blocks() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "Catalog",
            "--blocks",
            "uuid,uuid",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.success();
    dir.close().unwrap()
}

#[test]
fn cli_dissect_malformed() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/malformed",
            "--model",
            "Catalog",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.failure();
    dir.close().unwrap()
}

#[test]
fn cli_wrong_file_loc() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/not",
            "--model",
            "catalog",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.stderr("Error: Could not determine the full path of the OSCAL model file: `tests/data/not`\n\nCaused by:\n    No such file or directory (os error 2)\n");
    dir.close().unwrap()
}

#[test]
fn cli_wrong_dissect_model() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "invalid",
            "--blocks",
            "all",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.stderr("Invalid model provided: invalid\n");
    dir.close().unwrap()
}

#[test]
fn cli_wrong_dissect_block() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "Catalog",
            "--blocks",
            "uuid,not",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.stderr("Invalid catalog block provided: [\"uuid\", \"not\"]\n");
    dir.close().unwrap()
}

#[test]
fn cli_wrong_dissect_block_multiple_with_all() {
    let dir = tempdir().unwrap();
    let dir_arg = dir.as_ref().to_str().unwrap();
    let mut cmd = Command::cargo_bin("roscal").unwrap();
    let assert = cmd
        .args(&[
            "dissect",
            "--file",
            "tests/data/catalog.yaml",
            "--model",
            "Catalog",
            "--blocks",
            "all,uuid",
            "--output-dir",
            dir_arg,
        ])
        .assert();
    assert.stderr("Multiple block cannot include `all` option\n");
    dir.close().unwrap()
}
