use clap::Parser;
use glob::glob;
use std::{
    fmt::Display,
    fs::{File, canonicalize, read_dir},
    io::Read,
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "dirsize")]
#[command(
    about = "a simple command line utility to calculate the total size of a directory and its subdirectories"
)]
struct Args {
    path: PathBuf,
    #[arg(short, long, help = "Ignore paths in .gitignore (if it exists)")]
    gitignore: bool,
}

fn summ_size(root_dir: PathBuf, ignore_git: bool) -> Result<u64, Box<dyn std::error::Error>> {
    let ignore_list = if ignore_git {
        make_ignore_list(root_dir.clone())?
    } else {
        Vec::<PathBuf>::new()
    };
    // println!("ignore_list: {ignore_list:?}");

    let mut entries = read_dir(root_dir)?;
    let mut size: u64 = 0;
    while let Some(Ok(entry)) = entries.next() {
        let meta = entry.metadata()?;
        if ignore_list.contains(&entry.path()) {
            continue;
        }
        if meta.is_dir() {
            let path = entry.path();
            let sz = summ_size(path, ignore_git)?;
            size += sz;
        } else {
            size += meta.len();
        }
    }
    Ok(size)
}

enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::B => write!(f, "B"),
            Unit::KB => write!(f, "KB"),
            Unit::MB => write!(f, "MB"),
            Unit::GB => write!(f, "GB"),
            Unit::TB => write!(f, "TB"),
        }
    }
}

fn make_ignore_list(root_dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let gitignore_filepath = root_dir.join(".gitignore");
    // println!("gitignore_filepath: {:?}", &gitignore_filepath);
    let mut list = Vec::<PathBuf>::new();
    if gitignore_filepath.exists() {
        let mut f = File::open(gitignore_filepath)?;
        let mut text = String::new();
        f.read_to_string(&mut text)?;
        // println!("text: {text}");
        list = {
            let mut l = Vec::<PathBuf>::new();
            text.lines().for_each(|line| {
                if !line.starts_with(&['#', '!']) && !line.is_empty() {
                    let line = if line.starts_with('/') {
                        line[1..line.len()].to_owned()
                    } else {
                        line.to_owned()
                    };
                    // println!("{line:?}");
                    let path_buf = root_dir.join(line);
                    // println!("path_buf: {path_buf:?}");
                    if let Ok(canonical_path) = canonicalize(path_buf) {
                        let path_str = canonical_path.to_str().unwrap();
                        // println!("path_str: {path_str:?}");
                        for entry in glob(path_str).expect("Invalid glob pattern.") {
                            // println!("{entry:#?}");
                            match entry {
                                Ok(path) => {
                                    // println!("{path:?}");
                                    l.push(path.clone());
                                    // println!("{:?}", path.display());
                                }
                                Err(e) => println!("{:?}", e),
                            }
                        }
                    }
                }
            });

            l
        }
    }
    Ok(list)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let root_dir = canonicalize(args.path.as_path())?.clone();

    let size = summ_size(root_dir, args.gitignore)?;
    let unit = match size {
        0..=1023 => Unit::B,
        1024..=1_048_575 => Unit::KB,
        1_048_576..=1_073_741_823 => Unit::MB,
        1_073_741_824..=1_099_511_627_775 => Unit::GB,
        1_099_511_627_776.. => Unit::TB,
    };

    let size = size as f64;
    let fsize = match unit {
        Unit::KB => size / 1024.0,
        Unit::MB => size / 1_048_576.0,
        Unit::GB => size / 1_073_741_824.0,
        Unit::TB => size / 1_099_511_627_776.0,
        Unit::B => size,
    };

    println!("{fsize:.2} {unit}");

    Ok(())
}
