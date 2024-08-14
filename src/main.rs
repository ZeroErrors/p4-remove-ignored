use std::{path::Path, time::Instant};

use clap::Parser;
use p4::ignores;
use walkdir::WalkDir;

mod p4;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Perforce port to connect to
    #[arg(short, long)]
    port: String,

    /// The Perforce user to connect as
    #[arg(short, long)]
    user: String,

    /// The Perforce client to use
    #[arg(short, long)]
    client: String,

    #[arg(short, long)]
    dry_run: bool,

    /// The depot paths to remove ignored files from
    #[arg(required = true)]
    depot_paths: Vec<String>,
}

impl Args {
    fn to_p4_options(&self) -> p4::Options {
        p4::Options {
            port: self.port.clone(),
            user: self.user.clone(),
            client: self.client.clone(),
        }
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
                * 8,
        )
        .build_global()
        .unwrap();

    let args = Args::parse();
    let p4_options = args.to_p4_options();

    println!("Finding ignored files in paths: {:?}", args.depot_paths);

    // TODO: Run `p4 -Mj -z tag fstat -Rc -T clientFile <paths...>` to get all the files in the workspace

    println!("Finding files in workspace...");
    let start_time = Instant::now();
    let workspace_paths = p4::run_batched(
        |args| p4::fstat::run_clientfile(&p4_options, args),
        args.depot_paths,
    )
    .iter()
    .map(|output| output.client_file.clone())
    .collect::<Vec<_>>();
    println!(
        "Found {} files in {} seconds.",
        workspace_paths.len(),
        start_time.elapsed().as_secs_f32()
    );

    // Trim workspace paths to be relative to the workspace root so the paths are shorter and we need less batched commands
    let working_directory = std::env::current_dir().unwrap();
    let workspace_paths = workspace_paths
        .iter()
        .map(|path| {
            let path = Path::new(path);
            path.strip_prefix(&working_directory).unwrap_or(path)
        })
        .collect::<Vec<_>>();

    // TODO: Instead of running `p4 ignores -i` for all the files run `p4 ignores` to get the ignore rules and then match the files against the rules
    // p4:ignores::get_ignore_mappings(options).iter().map(|line| line.replace("...", "**"))

    // - Run `p4 -Mj -z tag ignores -i <path...>` to find ignored files
    println!("Finding ignored files...");
    let start_time = Instant::now();
    let ignored_files =
        p4::run_batched(|args| p4::ignores::run(&p4_options, args), workspace_paths);
    println!(
        "Found {} ignored files in {} seconds.",
        ignored_files.len(),
        start_time.elapsed().as_secs_f32()
    );

    // - Run `p4 -Mj -z tag delete -k <path...>` to delete files that should be ignored
    //    - If dry run just print the files that would be deleted
    if args.dry_run {
        println!("Would delete files:");
        for file in ignored_files {
            println!("{}", file);
        }
    } else {
        println!("Deleting files...");
        let start_time = Instant::now();
        p4::run_batched(|args| p4::delete::run(&p4_options, args), &ignored_files);
        println!(
            "Deleted {} files in {} seconds.",
            ignored_files.len(),
            start_time.elapsed().as_secs_f32()
        );
    }
}
