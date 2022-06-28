use std::env;

use clap::Parser;
use regex::Regex;
use std::path::PathBuf;

/// Find files using regex
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct FindApi {
    /// regex for files to search
    #[clap(value_parser)]
    regex: String,
    #[clap(short, long, value_parser)]
    /// Path to to start in
    path: Option<String>,
    /// If provided, print a lot
    #[clap(short, long, action)]
    verbose: bool,
}

fn main() {
    let find_api = FindApi::parse();
    let path: PathBuf = match &find_api.path {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().unwrap(),
    };
    let re = regex::Regex::new(&find_api.regex).unwrap();
    let matches = find_matches(path, &re, &find_api).unwrap();

    for re_match in matches.into_iter() {
        println!("{}", re_match);
    }
}

fn find_matches(
    path: PathBuf,
    re: &Regex,
    find_api: &FindApi,
) -> Result<Vec<String>, std::io::Error> {
    let mut matches: Vec<String> = vec![];

    for sub in std::fs::read_dir(path)? {
        // let sub_str = PathBuf::from(sub.unwrap().path()).to_str().unwrap();
        let sub_str = sub
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if re.is_match(&sub_str) {
            if find_api.verbose {
                println!("{}", sub_str);
            }
            matches.push(
                sub.as_ref()
                    .unwrap()
                    .path()
                    .as_path()
                    .to_owned()
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        if PathBuf::from(sub.as_ref().unwrap().path().as_path()).is_dir() {
            match find_matches(sub.as_ref().unwrap().path(), re, find_api) {
                Ok(sub_matches) => {
                    matches.extend(sub_matches);
                }
                Err(err) => {
                    if find_api.verbose {
                        println!("Error on {}: {}", sub_str, err);
                    }
                }
            }
        }
    }
    Ok(matches)
}
