use std::{ffi::OsStr, process::Command};

use serde::Deserialize;

/// The output of `p4 -Mj -z tag where <file...>`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub client_file: String,
    pub depot_file: String,
    pub path: String,
}

/// Runs `p4 -Mj -z tag where <file...>`
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
        .args(["-Mj", "-z", "tag", "where"])
        .args(paths)
        .output() // TODO: Stream the output to reduce the amount of buffering
        .expect("Failed to run p4 where");

    super::parse_output(&output.stdout)
}
