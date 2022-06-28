use std::env;

use clap::Parser;
use regex::Regex;
use std::path::PathBuf;

/// Find files using regex
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct FinderApi {
    /// regex for files to search
    #[clap(value_parser)]
    regex: String,
    #[clap(short, long, value_parser)]
    /// Optional path to to start in.  Defaults to current dir.  
    path: Option<String>,
}

fn main() {
    let finder_api = FinderApi::parse();
    let path: PathBuf = match &finder_api.path {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().unwrap(),
    };
    let re = regex::Regex::new(&finder_api.regex).unwrap();

    let _matches = find_matches(path, &re, &finder_api).unwrap();
}

fn find_matches(
    path: PathBuf,
    re: &Regex,
    finder_api: &FinderApi,
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
            println!(
                "{}",
                sub.as_ref()
                    .unwrap()
                    .path()
                    .as_path()
                    .to_owned()
                    .to_string_lossy()
                    .into_owned()
            );
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
            match find_matches(sub.as_ref().unwrap().path(), re, finder_api) {
                Ok(sub_matches) => {
                    matches.extend(sub_matches);
                }
                Err(err) => {
                    println!("Error on {}: {}", sub_str, err);
                }
            }
        }
    }
    Ok(matches)
}
