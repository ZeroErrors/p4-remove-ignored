use std::{ffi::OsStr, io::BufRead, process::Command};

use thiserror::Error;

pub type Output = String;

/// Runs `p4 ignores` to get the ignore mappings
pub fn get_ignore_mappings<I, S>(options: &super::Options) -> Vec<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("p4")
        .arg("-p")
        .arg(&options.port)
        .arg("-u")
        .arg(&options.user)
        .arg("-c")
        .arg(&options.client)
        .arg("ignores")
        .output() // TODO: Stream the output to reduce the amount of buffering
        .expect("Failed to run p4 where");

    // Note: `-Mj -z tag` doesn't work for p4 ignores so we need to manually parse the output
    output
        .stdout
        .lines()
        .filter_map(|line| line.ok()) // TODO: Log errors for decoding failures
        .collect()
}

/// Runs `p4 ignores -i <path...>`
pub fn run<I, S>(options: &super::Options, paths: I) -> Vec<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("p4")
        .arg("-p")
        .arg(&options.port)
        .arg("-u")
        .arg(&options.user)
        .arg("-c")
        .arg(&options.client)
        .args(["ignores", "-i"])
        .args(paths)
        .output() // TODO: Stream the output to reduce the amount of buffering
        .expect("Failed to run p4 where");

    // Note: `-Mj -z tag` doesn't work for p4 ignores so we need to manually parse the output
    output
        .stdout
        .lines()
        .filter_map(|line| line.ok()) // TODO: Log errors for decoding failures
        .map(|line| parse_output(&line))
        .filter_map(|line| line.ok()) // TODO: Log errors for parsing failures
        .collect()
}

/// Parses the output of `p4 -Mj -z tag ignores -i <path...>`
pub fn parse_output(line: &str) -> Result<Output, OutputParseError> {
    // Each line is a filepath followed by "ignored"
    // We need to be careful to handle paths with spaces so the easiest way is to parse from the end
    // eg.: `d:\github\EpicGames\UnrealEngine\UnrealEngine.generated.sln ignored`
    let mut parts = line.rsplitn(2, ' ');

    // The first part is the ignored flag
    let _ignored = parts.next().ok_or(OutputParseError::MissingIgnoredFlag)?;

    // The second part is the path
    let path = parts
        .next()
        .ok_or(OutputParseError::MissingPath)?
        .to_string();

    Ok(path)
}

#[derive(Error, Debug)]
pub enum OutputParseError {
    #[error("Missing ignored flag")]
    MissingIgnoredFlag,

    #[error("Missing path")]
    MissingPath,
}
