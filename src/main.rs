use clap::Parser;
use std::{
    fmt::Display,
    fs::{ReadDir, canonicalize, read_dir},
    path::PathBuf,
};

#[derive(Parser)]
#[command(about)]
struct Args {
    path: PathBuf,
}

fn summ_size(dir: &mut ReadDir) -> Result<u64, Box<dyn std::error::Error>> {
    let mut size: u64 = 0;
    while let Some(Ok(entry)) = dir.next() {
        let meta = entry.metadata()?;
        if meta.is_dir() {
            let path = entry.path();
            let mut dir = read_dir(path)?;
            let sz = summ_size(&mut dir)?;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = canonicalize(Args::parse().path.as_path())?;
    let mut entries = read_dir(path)?;
    let size = summ_size(&mut entries)?;
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
