mod app;
mod sys;
mod config;

use app::AppCommands;
use clap::{crate_description, crate_name, crate_version, value_parser};
use clap::{Arg, ArgMatches, Command};
use std::env;
use std::path::PathBuf;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        app::show_app_desc();
        std::process::exit(0);
    }
    let arg_matches: ArgMatches = parse_args();
    let subcommand_name = arg_matches.subcommand_name().unwrap_or("");
    let app_command = AppCommands::from_str(subcommand_name);

    match app_command {
        Some(AppCommands::Copy) => {
            
        }
        Some(AppCommands::Move) => {
            
        }
        Some(AppCommands::Delete) => {
            
        }
        Some(AppCommands::List) => {
            
        }
        Some(AppCommands::Batch) => {
            
        }
        None => {},
    }

}

fn parse_args() -> ArgMatches {
    let app: Command = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .allow_external_subcommands(true)
        .arg(Arg::new("log")
            .help("Save log Example: --log result.log")
            .long("log")
            .value_name("file_path")
            .value_parser(value_parser!(PathBuf))
        )
        .arg(Arg::new("quiet")
            .help("Quiet mode. Suppress output. Only show final results.")
            .short('q')
            .long("quiet")
            .num_args(0)
        )
        .subcommand(Command::new("copy")
            .about("Copy files or directories.")
            .arg(Arg::new("src")
                .help("Specify the source file or directory")
                .long("src")
                .short('s')
                .value_name("path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(Arg::new("dst")
                .help("Specify the ports. Example: 80,443,8080")
                .long("dst")
                .short('d')
                .value_name("path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(Arg::new("recursive")
                .help("Copy directories recursively")
                .long("recursive")
                .short('r')
                .num_args(0)
            )
            .arg(Arg::new("age")
                .help("Set age in hours for files to be copied (default: 0) - Example: --age 24")
                .long("age")
                .value_name("age_hours")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("move")
            .about("Move files or directories.")
            .arg(Arg::new("src")
                .help("Specify the source file or directory")
                .long("src")
                .short('s')
                .value_name("path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(Arg::new("dst")
                .help("Specify the ports. Example: 80,443,8080")
                .long("dst")
                .short('d')
                .value_name("path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(Arg::new("recursive")
                .help("Copy directories recursively")
                .long("recursive")
                .short('r')
                .num_args(0)
            )
            .arg(Arg::new("age")
                .help("Set age in hours for files to be copied (default: 0) - Example: --age 24")
                .long("age")
                .value_name("age_hours")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("delete")
            .about("Delete files or directories.")
            .arg(Arg::new("target")
                .help("Specify the target file or directory to delete")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("recursive")
                .help("Copy directories recursively")
                .long("recursive")
                .short('r')
                .num_args(0)
            )
            .arg(Arg::new("age")
                .help("Set age in hours for files to be copied (default: 0) - Example: --age 24")
                .long("age")
                .value_name("age_hours")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("list")
            .about("List files or directories.")
            .arg(Arg::new("target")
                .help("Specify the target file or directory to list")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("recursive")
                .help("Copy directories recursively")
                .long("recursive")
                .short('r')
                .num_args(0)
            )
            .arg(Arg::new("age")
                .help("Set age in hours for files to be copied (default: 0) - Example: --age 24")
                .long("age")
                .value_name("age_hours")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("batch")
            .about("Run batch operations from a configuration file.")
            .arg(Arg::new("config")
                .help("Path to the batch configuration file")
                .long("config")
                .short('c')
                .value_name("config_path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
        )
        ;
    app.get_matches()
}
