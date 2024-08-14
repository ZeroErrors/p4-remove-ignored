use std::{ffi::OsStr, io::BufRead};

use rayon::prelude::*;

pub struct Options {
    pub port: String,
    pub user: String,
    pub client: String,
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
        if batch_size + arg_size > 1024 {
            batches.push(std::mem::take(&mut batch));
            batch_size = 0;
        }

        batch.push(arg);
        batch_size += arg_size;
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
