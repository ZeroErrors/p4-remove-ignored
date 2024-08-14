use std::{ffi::OsStr, process::Command};

use serde::Deserialize;

/// The output of `p4 -Mj -z tag files -e <depot-path>`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub action: String,
    pub change: String,
    pub depot_file: String,
    pub rev: String,
    pub time: String,
    #[serde(rename = "type")]
    pub file_type: String,
}

/// Runs `p4 -Mj -z tag files -e <depot-path>`
pub fn run<S: AsRef<OsStr>>(options: &super::Options, depot_path: S) -> Vec<Output> {
    let mut command = Command::new("p4");
    options.append_args(&mut command);
    let output = command
        .args(["-Mj", "-z", "tag", "files", "-e"])
        .arg(depot_path)
        .output() // TODO: Stream the output to reduce the amount of buffering
        .expect("Failed to run p4 files");

    super::parse_output(&output.stdout)
}
