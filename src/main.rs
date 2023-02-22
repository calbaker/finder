use clap::Parser;
use regex::Regex;
use std::env;
use std::path::PathBuf;

/// Find files using regex.  Put this in any folder (e.g. HOME/.finder/)
/// and add that folder to your user `Path` to use from any terminal.  
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
    let mut dirs: Vec<PathBuf> = vec![];

    for sub in std::fs::read_dir(path)? {
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
            dirs.push(PathBuf::from(sub.as_ref().unwrap().path().as_path()));
        }
    }

    for dir in dirs {
        match find_matches(dir.clone(), re, finder_api) {
            Ok(sub_matches) => {
                matches.extend(sub_matches);
            }
            Err(err) => {
                println!(
                    "Error on {}: {}",
                    dir.canonicalize()
                        .unwrap()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                    err
                );
            }
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    pub fn test_that_cargo_toml_is_found() {
        let output = Command::new("./target/release/finder.exe")
            .args(&["argo"])
            .output()
            .unwrap_or(
                Command::new("./target/release/finder") // for linux/mac
                    .args(&["argo"])
                    .output()
                    .expect("run `cargo build --release`"),
            );
        println!("output: {:?}", String::from_utf8(output.to_owned().stdout));
        let re = regex::Regex::new("argo").unwrap();
        assert!(re.is_match(&String::from_utf8(output.stdout).unwrap()));
    }
}
