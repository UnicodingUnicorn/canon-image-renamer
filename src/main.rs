mod datetime;
use datetime::{ PhotoDate, DateTimeError };

use std::collections::HashMap;
use std::fs::{ self, File };
use std::io::BufReader;
use std::path::Path;

use clap::Parser;
use thiserror::Error;

#[derive(Debug, Parser)]
struct Args {
    #[clap(value_parser)]
    input_directory:String,
    /// Output directory
    #[clap(short, long, value_parser, default_value="./output")]
    output:String,
    /// Starting index for the filenames
    #[clap(short, long, value_parser, default_value_t=1)]
    start:usize,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = process_directory(&args.input_directory, &args.output, args.start) {
        println!("{}", e);
        std::process::exit(1);
    }
}

#[derive(Debug, Error)]
enum ProcessError {
    #[error("Error reading file: {0}")]
    IO(#[from] std::io::Error),
    #[error("Error parsing EXIF: {0}")]
    EXIF(#[from] exif::Error),
    #[error("Error parsing exif datetime: {0}")]
    DateTime(#[from] DateTimeError),
    #[error("Duplicate time entry existss")]
    Duplicate,
    #[error("Entries in cache do not match up. This should not happen")]
    CacheMismatch,
}

fn process_directory(directory:&str, output_directory:&str, start:usize) -> Result<(), ProcessError> {
    let exifreader = exif::Reader::new();
    create_if_not_exists(output_directory)?;

    let mut cache:HashMap<PhotoDate, HashMap<String, String>> = HashMap::new();
    let mut sort_list = Vec::new();

    let paths = fs::read_dir(directory)?;
    for path in paths {
        let path = path?.path();

        let file = File::open(&path)?;
        let mut reader = BufReader::new(&file);
        let exif = exifreader.read_from_container(&mut reader)?;

        let photo_date = PhotoDate::new(&exif)?;
        let extension = match path.extension() {
            Some(ext) => format!(".{}", ext.to_string_lossy()),
            None => String::new(),
        };

        match cache.get_mut(&photo_date) {
            Some(extensions) => extensions.insert(extension, path.to_string_lossy().to_string()).map_or(Ok(()), |_| Err(ProcessError::Duplicate))?,
            None => {
                let mut extensions = HashMap::new();
                let _ = extensions.insert(extension, path.to_string_lossy().to_string());
                let _ = cache.insert(photo_date, extensions);
            },
        };

        sort_list.push(photo_date);
    }

    sort_list.sort();
    sort_list.dedup();

    for (i, photo_date) in sort_list.iter().enumerate() {
        let files = cache.get(&photo_date).ok_or(ProcessError::CacheMismatch)?;

        let folder_name = photo_date.folder_name();
        let output_path = Path::new(output_directory).join(&folder_name);
        create_if_not_exists(&output_path)?;

        for (ext, original_path) in files.iter() {
            fs::copy(&original_path, &output_path.join(format!("IMG_{}{}", start + i, ext)))?;
        }
    }

    Ok(())
}

fn create_if_not_exists<P:AsRef<Path>>(dir:P) -> std::io::Result<()> {
    let exists = match fs::metadata(&dir) {
        Ok(_) => true,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => false,
            _ => return Err(e),
        },
    };

    if !exists {
        fs::create_dir(&dir)?;
    }

    Ok(())
}
