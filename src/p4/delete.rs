use std::{ffi::OsStr, process::Command};

use serde::Deserialize;

/// The output of `p4 -Mj -z tag delete -k <path...>`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub action: String,
    pub client_file: String,
    pub depot_file: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub work_rev: String,
}

/// Runs `p4 -Mj -z tag delete -k <path...>`
pub fn run<I, S>(options: &super::Options, keep_files: bool, paths: I) -> Vec<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("p4");
    command
        .arg("-p")
        .arg(&options.port)
        .arg("-u")
        .arg(&options.user)
        .arg("-c")
        .arg(&options.client)
        .args(["-Mj", "-z", "tag", "delete"]);
    if keep_files {
        command.arg("-k");
    }
    let output = command
        .args(paths)
        .output() // TODO: Stream the output to reduce the amount of buffering
        .expect("Failed to run p4 delete");

    super::parse_output(&output.stdout)
}
