#![allow(clippy::single_char_pattern)]
use std::{fs::File, io::Read, path::Path};

use anyhow::{Context, Result};
use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha2::{Digest, Sha256};

pub(super) fn check_filename_len(path: impl AsRef<Path>) -> bool {
    if path.as_ref().to_string_lossy().as_bytes().len() > 202 {
        eprintln!("File name is too long");
        return false;
    }

    true
}

pub(super) fn is_valid_file(path: impl AsRef<Path>) -> bool {
    if check_filename_len(&path) && !path.as_ref().is_file() {
        eprintln!("Invalid file provided: {}", path.as_ref().display());
        return false;
    }

    true
}

pub(super) fn is_valid_dir(path: impl AsRef<Path>) -> bool {
    if !path.as_ref().is_dir() {
        eprintln!(
            "Invalid directory provided: {}",
            path.as_ref().to_string_lossy()
        );
        return false;
    }

    true
}

pub(super) fn gen_created_at() -> String {
    let now = Utc::now().to_string();

    now.as_str().replace(" ", "_")
}

pub(super) fn gen_rand() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

pub(super) fn gen_hash_from_path(path: &impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(path).with_context(|| {
        format!("Could not open file: `{}`", path.as_ref().display())
    })?;

    let mut hasher = Sha256::new();

    let mut buffer = [0; 4096];

    loop {
        let bytes_read = file.read(&mut buffer).with_context(|| {
            format!("Could not read file: `{}`", path.as_ref().display())
        })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub(super) fn gen_hash_from_str(content: &str) -> Result<String> {
    let mut hasher = Sha256::new();

    hasher.update(content.as_bytes());

    Ok(format!("{:x}", hasher.finalize()))
}
