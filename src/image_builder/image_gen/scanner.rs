use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use phf::phf_map;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{collections::HashSet, fs, path::Path, sync::Mutex};
use std::{fs::DirEntry, io};
use thiserror::Error;

lazy_static! {
    static ref IS_READY: Mutex<bool> = Mutex::new(false);
    static ref FILETYPES: Mutex<ImageInfo> = Mutex::new(ImageInfo {
        file_types: HashSet::new(),
    });
    static ref FILE_TYPE_LIST: Option<HashMap<String,String>> = {
        let mut map = HashMap::new();

        // Load the first file
        if let Ok(json_data) = fs::read_to_string("langs.json").context("Failed to read langs.json") {
            if let Ok(data) = serde_json::from_str::<HashMap<String, String>>(&json_data).context("failed to map langs.json into a hashmap") {
                map.extend(data);
            }
        }

        // Load the second file and override values
        if let Ok(json_data) = fs::read_to_string(".forge_override.json").context("Failed to read override_langs.json") {
            if let Ok(data) = serde_json::from_str::<HashMap<String, String>>(&json_data).context("failed to map override_langs.json into a hashmap") {
                map.extend(data);
            }
        }

        Some(map)
    };
} 

#[derive(Debug)]
pub struct ImageInfo {
    pub file_types: HashSet<String>,
}

#[derive(Debug, Error)]
enum CompileError {
    #[error("could not read path: {0}")]
    InvalidPath(String),
    #[error("IO error occurred: {0}")]
    IoError(#[from] io::Error),
}

pub fn scan(input_path: &str) -> Result<Vec<String>>  {
    let mut types: HashSet<String> = HashSet::new();

    let path = Path::new(input_path);
    if path.exists() {
        types = get_file_types(path.into())?;
        let mut type_urls: Vec<String> = Vec::new();
        for filetype in types.clone() {
            if let Some(url) = FILE_TYPE_LIST.as_ref().and_then(|map| map.get(&filetype)) {
                type_urls.push(url.clone());
            }
        }
        println!("{:?}", type_urls);
        Ok(type_urls)
    } else {
        Err(anyhow!("Path does not exist"))
    }

}

fn get_file_types(path: PathBuf) -> Result<HashSet<String>> {
    let mut file_types = HashSet::new();
    walk_dir(&path, test_callback).context("failed to walk directory")?;
    match FILETYPES.lock() {
        std::result::Result::Ok(types) => {
            println!("{:#?}", types.file_types);
            file_types = types.file_types.clone()
        }
        Err(e) => {
            println!("{e}")
        }
    }
    println!("File types collected.");
    Ok(file_types)
}

fn walk_dir(input_dir: &PathBuf, callback: fn(&DirEntry)) -> Result<()> {
    if !input_dir.is_dir() {
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(|entry| match entry {
            std::result::Result::Ok(e) => Some(e),
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
                None
            }
        })
        .collect();

    entries.par_iter().try_for_each::<_, Result<(), anyhow::Error>>(|entry: &DirEntry| {
        let path = entry.path();
        if path.is_dir() {
            if let Err(err) = walk_dir(&path, callback) {
                eprintln!("Error walking directory {}: {}", path.display(), err);
            }
        } else {
            callback(entry);
        }
        Ok(())
    })?;
    Ok(())
}

fn test_callback(foo: &DirEntry) {
    if let Some(extension) = foo.path().extension() {
        if let Some(ext_str) = extension.to_str() {
            if let std::result::Result::Ok(mut filetypes) = FILETYPES.lock() {
                filetypes.file_types.insert(ext_str.to_string());
            } else {
                eprintln!("Failed to lock FILETYPES mutex.");
            }
        }
    }
}
