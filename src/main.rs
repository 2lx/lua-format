use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod config;
mod file_util;
mod formatting;
mod parser;
use config::{Config, ConfiguredWrite};

fn get_options_and_filenames() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let (options, mut sources): (Vec<_>, Vec<_>) = args.into_iter().partition(|arg| arg.starts_with('-'));
    sources.sort();

    (options, sources)
}

#[derive(Debug)]
pub struct ProgramOpts {
    pub inplace: bool,
    pub recursive: bool,
}

impl ProgramOpts {
    pub const fn default() -> Self {
        ProgramOpts { inplace: false, recursive: false }
    }
}

fn parse_options(options: &Vec<String>) -> (Config, ProgramOpts) {
    let mut config = Config::default();
    let mut program_opts = ProgramOpts::default();

    for option in options.iter() {
        let re_config_opt = Regex::new(r"^[-]+([a-zA-Z_0-9]+)\s*=(.*)$").unwrap();
        let re_program_opt = Regex::new(r"^[-]+([a-zA-Z_0-9]+)$").unwrap();

        match re_config_opt.captures_iter(option).next() {
            Some(cap) => config.set(&cap[1], &cap[2]),
            None => match re_program_opt.captures_iter(option).next() {
                Some(cap) if &cap[1] == "i" || &cap[1] == "inplace" => program_opts.inplace = true,
                Some(cap) if &cap[1] == "r" || &cap[1] == "recursive" => program_opts.recursive = true,
                _ => eprintln!("Unrecognized option `{}`", option),
            },
        }
    }

    (config, program_opts)
}

fn process_file(file_path: &PathBuf, config: &Config, program_opts: &ProgramOpts) {
    println!("Process file: `{}`", file_path.display());
    if config.is_empty() {
        match file_util::get_file_config(file_path) {
            Some(file_config) => {
                let cfg = Config::load_from_file(&file_config);
                process_file_with_config(&file_path, &cfg, &program_opts);
            }
            None => println!("Configure file was not found"),
        }
    } else {
        process_file_with_config(&file_path, &config, &program_opts);
    }
}

fn process_file_with_config(file_path: &PathBuf, config: &Config, program_opts: &ProgramOpts) {
    println!("Format options: {}", config);

    let content =
        fs::read_to_string(file_path).expect(&format!("An error occured while reading file `{}`", file_path.display()));

    match parser::parse_lua(&content) {
        Ok(node_tree) => {
            let mut outbuffer = String::new();
            let mut state = config::State::default();
            match node_tree.configured_write(&mut outbuffer, &config, &content, &mut state) {
                Ok(_) => match program_opts.inplace {
                    true => fs::write(file_path, outbuffer)
                        .expect(&format!("An error occured while writing file `{}`", file_path.display())),
                    false => print!("\n{}", outbuffer),
                },
                Err(_) => println!("An error occured while formatting file `{}`: {:?}", file_path.display(), node_tree),
            };
        }
        Err(err) => println!("An error occured while parsing file `{}`: {}", file_path.display(), err),
    }
}

fn main() {
    let (options, rel_paths) = get_options_and_filenames();
    let (config, program_opts) = parse_options(&options);

    println!("Paths: {:?}", rel_paths);
    println!("Program options: {:?}", program_opts);

    for rel_path in &rel_paths {
        let path_buf = Path::new(rel_path).to_path_buf();

        match file_util::get_path_files(&path_buf, program_opts.recursive) {
            Ok(file_paths) => {
                for file_path in &file_paths {
                    process_file(&file_path, &config, &program_opts);
                }
            }
            Err(_) => println!("Unresolved path: `{}`", rel_path),
        }
    }
}
