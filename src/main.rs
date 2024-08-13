use std::time::Instant;

use clap::Parser;

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

    println!(
        "Finding ignored files in depot paths: {:?}",
        args.depot_paths
    );

    // - Run `p4 -Mj -z tag files -e <depot-path>` to find all files recurisvly for each depot path provided
    println!("Finding files in depot paths...");
    let start_time = Instant::now();
    let depot_paths = args
        .depot_paths
        .iter()
        .flat_map(|path| p4::files::run(&p4_options, path))
        .map(|file| file.depot_file)
        .collect::<Vec<_>>();
    println!(
        "Found {} files in {} seconds.",
        depot_paths.len(),
        start_time.elapsed().as_secs_f32()
    );

    // - Run `p4 -Mj -z tag where <file...>` to transform from depot paths to workspace paths
    println!("Mapping workspace paths...");
    let start_time = Instant::now();
    let workspace_paths = p4::run_batched(|args| p4::where_::run(&p4_options, args), depot_paths)
        .iter()
        .map(|file| file.path.to_string())
        .collect::<Vec<_>>();
    println!("Took {} seconds.", start_time.elapsed().as_secs_f32());

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
        p4::delete::run(&p4_options, &ignored_files);
        println!(
            "Deleted {} files in {} seconds.",
            ignored_files.len(),
            start_time.elapsed().as_secs_f32()
        );
    }
}
