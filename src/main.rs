/*
 * Stray thoughts
 * * File system abstraction -- either system paths, or maybe an S3 bucket as some kind of
 *   middle layer
 *
 * What are the things that define sources
 * * source name -- all custom
 * * file name -- source name by default, would be nice to tailor by source
 * * filetype -- could be enumerated, would be nice to specify at runtime eventually
 * * version pattern: semver, ISO-8601, custom
 * * version check: custom or from github release tag
 * * download method: http, ftp, github release?
 * * unpack method: gz, tarball, biggest file in zip, other?
 *
 * * is an iterable remote version available? maybe a struct for remote version access
 *
 * * Meaningful traits to try
 * * basic DataSource
 * * GitHub data source
 *
 * some kind of abstraction for file version? support sorting, unversioned-ness (though this might
 * avoid this layer entirely)
 */

use clap::{Args, Parser, Subcommand};
use std::{fmt::Display, fs};

use crate::providers::data_provider::get_provider;
mod providers;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Dir,
    Get(GetArgs),
    Clean(CleanArgs),
    ListLocal(ListLocalArgs),
    ListRemote(ListRemoteArgs),
}

#[derive(Args)]
struct GetArgs {
    sources: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    silent: bool,
    #[arg(short, long, default_value_t = false)]
    from_local: bool,
    #[arg(short, long, default_value_t = false)]
    refresh: bool,
}

#[derive(Args)]
struct CleanArgs {
    sources: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    silent: bool,
    #[arg(short, long, default_value_t = false)]
    from_local: bool,
    #[arg(short, long, default_value_t = false)]
    refresh: bool,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
enum ListLocalPrintMode {
    #[default]
    Filename,
    Version,
    Path,
}

impl Display for ListLocalPrintMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListLocalPrintMode::Filename => write!(f, "filename"),
            ListLocalPrintMode::Version => write!(f, "version"),
            ListLocalPrintMode::Path => write!(f, "path"),
        }
    }
}

#[derive(Args, Debug)]
struct ListLocalArgs {
    source: String,
    #[arg(short, long, default_value_t = ListLocalPrintMode::Filename)]
    mode: ListLocalPrintMode,
}

fn list_local(args: &ListLocalArgs) {
    if let Ok(provider) = get_provider(&args.source) {
        if let Ok(dir) = provider.get_data_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                match args.mode {
                    ListLocalPrintMode::Version => {
                        let file_name = entry.file_name();
                        let caps = provider
                            .file_pattern()
                            .captures(file_name.to_str().unwrap())
                            .unwrap();
                        println!("{}", caps.get(1).unwrap().as_str())
                    }
                    ListLocalPrintMode::Path => println!("{}", entry.path().display()),
                    ListLocalPrintMode::Filename => {
                        println!("{}", entry.file_name().to_str().unwrap())
                    }
                }
            }
        }
    }
}

#[derive(Args)]
struct ListRemoteArgs {
    source: String,
}

fn dir() -> () {
    match utils::storage::get_wags_tails_dir() {
        Ok(path) => println!("{}", path.into_os_string().into_string().unwrap()),
        Err(descr) => {
            eprintln!("{}", descr);
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dir => dir(),
        Commands::Get(s) => println!("get {:?}", s.sources),
        Commands::Clean(_) => println!("clean"),
        Commands::ListLocal(args) => list_local(args),
        Commands::ListRemote(_) => println!("list remote"),
    }
}
