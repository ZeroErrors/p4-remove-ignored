use std::{ffi::OsStr, io::BufRead};

use rayon::prelude::*;

pub struct Options {
    pub port: Option<String>,
    pub user: Option<String>,
    pub client: Option<String>,
}

impl Options {
    pub fn append_args(&self, command: &mut std::process::Command) {
        if let Some(port) = &self.port {
            command.arg("-p").arg(port);
        }
        if let Some(user) = &self.user {
            command.arg("-u").arg(user);
        }
        if let Some(client) = &self.client {
            command.arg("-c").arg(client);
        }
    }
}

/// Run a command in batches to avoid the maximum command line length
pub fn run_batched<Output, F, I, S>(f: F, args: I) -> Vec<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr> + Send + Sync,
    F: Fn(&Vec<S>) -> Vec<Output> + Send + Sync,
    Output: Send,
{
    let mut batches = Vec::new();
    let mut batch = Vec::new();
    let mut batch_size = 0;

    for arg in args {
        let arg_size = arg.as_ref().len();
        const WINDOWS_MAX_ARG_SIZE: usize = 32768;
        const MAX_COMMAND_SIZE: usize = 1024; // Our maximum command size without arguments
        if batch_size + arg_size > WINDOWS_MAX_ARG_SIZE - MAX_COMMAND_SIZE {
            batches.push(std::mem::take(&mut batch));
            batch_size = 0;
        }

        batch.push(arg);
        batch_size += arg_size;
    }
    // Add the remaining batch
    if !batch.is_empty() {
        batches.push(std::mem::take(&mut batch));
    }

    println!("Running {} batches", batches.len());
    batches.par_iter().map(f).flatten().collect()
}

pub fn parse_output<T>(output: &[u8]) -> Vec<T>
where
    T: serde::de::DeserializeOwned,
{
    output
        .lines()
        .filter_map(|line| line.ok()) // TODO: Log errors for decoding failures
        .map(|line| serde_json::from_str::<T>(&line))
        .filter_map(|line| line.ok()) // TODO: Log errors for parsing failures
        .collect()
}

pub mod delete;
pub mod files;
pub mod fstat;
pub mod ignores;
pub mod where_;
