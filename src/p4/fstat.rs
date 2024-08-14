use std::{ffi::OsStr, process::Command};

use serde::Deserialize;

/// The output of `p4 -Mj -z tag fstat -Rc -T clientFile <paths...>`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputClientFile {
    pub client_file: String,
}

/// Runs `p4 -Mj -z tag fstat -Rc -T clientFile <paths...>`
pub fn run_clientfile<I, S>(options: &super::Options, paths: I) -> Vec<OutputClientFile>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("p4");
    options.append_args(&mut command);
    let output = command
        .args(["-Mj", "-z", "tag", "fstat", "-Rc", "-T", "clientFile"])
        .args(paths)
        .output() // TODO: Stream the output to reduce the amount of buffering
        .expect("Failed to run p4 fstat");

    super::parse_output(&output.stdout)
}
