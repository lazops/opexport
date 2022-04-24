use std::{io, process::Command};

use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum OPError {
    CommandError(io::Error),
    DeserializeError(serde_json::Error),
    CLIError(String),
}

// Executes op with JSON output using the passed arguments, and returns the parsed JSON.
pub fn run<T: DeserializeOwned>(args: &[&str]) -> Result<T, OPError> {
    let output = Command::new("op")
        .args(args)
        .arg("--cache")
        .arg("--format")
        .arg("json")
        .output()
        .map_err(OPError::CommandError)?;

    if output.stderr.len() > 0 {
        return Err(OPError::CLIError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(OPError::DeserializeError)
}
