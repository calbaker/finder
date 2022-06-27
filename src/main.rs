use std::env;

use clap::Parser;
use regex;
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
    let path: PathBuf = match find_api.path {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().unwrap(),
    };
    let re = regex::Regex::new(&find_api.regex).unwrap();
    let mut matches: Vec<String> = vec![];
    for sub in std::fs::read_dir(path).unwrap() {
        // let sub_str = PathBuf::from(sub.unwrap().path()).to_str().unwrap();
        let sub_str = sub
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
            matches.push(sub_str);
        }
    }
    println!("{:?}", matches);

    //     if sub.is_dir():
    //         try:
    //             matches.extend(find_file_regex(re_pattern, sub))
    //         except PermissionError:
    //             if verbose: print(f'Access is denied: {sub}')
    //         except OSError:
    //             if verbose: print(f'Access is denied: {sub}')
    //     elif re.search(re_pattern, str(sub.name)):
    //         if verbose: print(f'found match: {sub}')
    //         matches.append(str(sub))
    // return matches
}
