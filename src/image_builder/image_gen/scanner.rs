use anyhow::{anyhow, Context, Ok, Result};
use lazy_static::lazy_static;
use phf::phf_map;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
use std::{collections::HashSet, fs, path::Path, sync::Mutex};
use std::{fs::DirEntry, io};
use thiserror::Error;



lazy_static! {
    static ref IS_READY: Mutex<bool> = Mutex::new(false);
    static ref FILETYPES: Mutex<ImageInfo> = Mutex::new(ImageInfo {
        file_types: HashSet::new(),
    });
}

static FILE_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    // General-purpose programming languages
    "rs" => "ghcr.io/devcontainers/features/rust:latest",
    "py" => "ghcr.io/devcontainers/features/python:latest",
    // "c" => "ghcr.io/devcontainers/features/gcc:latest",
    // "h" => "ghcr.io/devcontainers/features/gcc:latest",
    // "cpp" => "ghcr.io/devcontainers/features/gcc:latest",
    // "hpp" => "ghcr.io/devcontainers/features/gcc:latest",
    "java" => "ghcr.io/devcontainers/features/java:latest",
    "js" => "ghcr.io/devcontainers/features/node:latest",
    "ts" => "ghcr.io/devcontainers/features/node:latest",
    "tsx" => "ghcr.io/devcontainers/features/node:latest",
    "jsx" => "ghcr.io/devcontainers/features/node:latest",
    "go" => "ghcr.io/devcontainers/features/go:latest",
    "rb" => "ghcr.io/devcontainers/features/ruby:latest",
    "php" => "ghcr.io/devcontainers/features/php:latest",
    "swift" => "ghcr.io/devcontainers/features/swift:latest",
    "kt" => "ghcr.io/devcontainers/features/kotlin:latest",
    "kts" => "ghcr.io/devcontainers/features/kotlin:latest",
    "cs" => "ghcr.io/devcontainers/features/dotnet:latest",
    "vb" => "ghcr.io/devcontainers/features/dotnet:latest",
    "fs" => "ghcr.io/devcontainers/features/dotnet:latest",
    "scala" => "ghcr.io/devcontainers/features/scala:latest",
    "pl" => "ghcr.io/devcontainers-contrib/features/perl-asdf:latest",
    "pm" => "ghcr.io/devcontainers/features/perl:latest",
    "r" => "ghcr.io/devcontainers/features/r:latest",
    "jl" => "ghcr.io/devcontainers/features/julia:latest",
    "sh" => "ghcr.io/devcontainers/features/bash:latest",
    "bat" => "ghcr.io/devcontainers/features/cmd:latest",
    "ps1" => "ghcr.io/devcontainers-contrib/features/powershell:latest",
    "lua" => "ghcr.io/devcontainers/features/lua:latest",

    // Functional programming languages
    "hs" => "ghcr.io/devcontainers/features/haskell:latest",
    "ml" => "ghcr.io/devcontainers/features/ocaml:latest",
    "elm" => "ghcr.io/devcontainers/features/node:latest",
    "clj" => "ghcr.io/devcontainers/features/clojure:latest",
    "cljs" => "ghcr.io/devcontainers/features/node:latest",

    // Scripting languages
    "tcl" => "ghcr.io/devcontainers/features/tcl:latest",
    "awk" => "ghcr.io/devcontainers/features/awk:latest",
    "sed" => "ghcr.io/devcontainers/features/sed:latest",

    // Shell-related files
    "zsh" => "ghcr.io/devcontainers/features/zsh:latest",
    "bash" => "ghcr.io/devcontainers/features/bash:latest",
    "fish" => "ghcr.io/devcontainers/features/fish:latest",

    // Web technologies and domain-specific
    "html" => "ghcr.io/devcontainers/features/node:latest",
    "css" => "ghcr.io/devcontainers/features/node:latest",
    "scss" => "ghcr.io/devcontainers/features/node:latest",
    "sass" => "ghcr.io/devcontainers/features/node:latest",
    "json" => "ghcr.io/devcontainers/features/node:latest",
    "yaml" => "ghcr.io/devcontainers/features/node:latest",
    "yml" => "ghcr.io/devcontainers/features/node:latest",
    "xml" => "ghcr.io/devcontainers/features/node:latest",
    "astro" => "ghcr.io/devcontainers/features/node:latest",
    "svelte" => "ghcr.io/devcontainers/features/node:latest",
    "vue" => "ghcr.io/devcontainers/features/node:latest",
    "mdx" => "ghcr.io/devcontainers/features/node:latest",

    // Text processing and markup
    "md" => "ghcr.io/devcontainers/features/node:latest",
    "rst" => "ghcr.io/devcontainers/features/python:latest",
    "tex" => "ghcr.io/devcontainers/features/tex:latest",

    // Hardware description and low-level
    "v" => "ghcr.io/devcontainers/features/verilog:latest",
    "sv" => "ghcr.io/devcontainers/features/systemverilog:latest",
    "vhdl" => "ghcr.io/devcontainers/features/ghdl:latest",
    "asm" => "ghcr.io/devcontainers/features/nasm:latest",
    "s" => "ghcr.io/devcontainers/features/gas:latest",
    "ada" => "ghcr.io/devcontainers/features/gnat:latest",
    "f90" => "ghcr.io/devcontainers/features/gfortran:latest",
    "f95" => "ghcr.io/devcontainers/features/gfortran:latest",
    "f03" => "ghcr.io/devcontainers/features/gfortran:latest",
    "f08" => "ghcr.io/devcontainers/features/gfortran:latest",

    // Template engines and preprocessors
    "pug" => "ghcr.io/devcontainers/features/node:latest",
    "hbs" => "ghcr.io/devcontainers/features/node:latest",
    "ejs" => "ghcr.io/devcontainers/features/node:latest",
    "njk" => "ghcr.io/devcontainers/features/node:latest",

    // Emerging or less common but widely used
    "dart" => "ghcr.io/devcontainers/features/dart:latest",
    "nim" => "ghcr.io/devcontainers/features/nim:latest",
    "zig" => "ghcr.io/devcontainers/features/zig:latest",
    "cr" => "ghcr.io/devcontainers/features/crystal:latest",
    "groovy" => "ghcr.io/devcontainers/features/groovy:latest"
};

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

pub fn scan(input_path: &str) -> Result<Vec<&'static str>>  {

    let path = Path::new(input_path);
    try_compile(path.into())?;
}

fn try_compile(path: PathBuf) -> Result<Mutex<ImageInfo>> {
    let mut file_types = HashSet::new();
    if path.exists() {
        walk_dir(&path, test_callback).context("failed to walk directory")?;
        match FILETYPES.lock() {
            std::result::Result::Ok(types) => {
                file_types = types.file_types
            }
            Err(e) => {}
        }

        println!("File types collected.");
        Ok(file_types)
    } else {
        Err(CompileError::InvalidPath(path.display().to_string()).into())
    }
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

    entries.par_iter().try_for_each(|entry: &DirEntry| {
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
