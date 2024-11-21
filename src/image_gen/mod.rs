use anyhow::{anyhow, Context, Ok, Result};
use lazy_static::lazy_static;
use phf::phf_map;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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
    "rs" => "rust",
    "py" => "python",
    "c" => "gcc",
    "h" => "gcc",
    "cpp" => "g++",
    "hpp" => "g++",
    "java" => "java",
    "js" => "node",
    "ts" => "node",
    "tsx" => "node",
    "jsx" => "node",
    "go" => "go",
    "rb" => "ruby",
    "php" => "php",
    "swift" => "swift",
    "kt" => "kotlin",
    "kts" => "kotlin",
    "cs" => "dotnet",
    "vb" => "dotnet",
    "fs" => "dotnet",
    "scala" => "scala",
    "pl" => "perl",
    "pm" => "perl",
    "r" => "r",
    "jl" => "julia",
    "sh" => "sh",
    "bat" => "cmd",
    "ps1" => "powershell",
    "lua" => "lua",

    // Functional programming languages
    "hs" => "ghc",
    "ml" => "ocaml",
    "elm" => "node",
    "clj" => "clojure",
    "cljs" => "node",

    // Scripting languages
    "tcl" => "tcl",
    "awk" => "awk",
    "sed" => "sed",

    // Shell-related files
    "zsh" => "zsh",
    "bash" => "bash",
    "fish" => "fish",

    // Web technologies and domain-specific
    "html" => "node",
    "css" => "node",
    "scss" => "node",
    "sass" => "node",
    "json" => "node",
    "yaml" => "node",
    "yml" => "node",
    "xml" => "node",
    "astro" => "node",
    "svelte" => "node",
    "vue" => "node",
    "mdx" => "node",

    // Text processing and markup
    "md" => "node",
    "rst" => "python",
    "tex" => "tex",

    // Hardware description and low-level
    "v" => "verilog",
    "sv" => "systemverilog",
    "vhdl" => "ghdl",
    "asm" => "nasm",
    "s" => "gas",
    "ada" => "gnat",
    "f90" => "gfortran",
    "f95" => "gfortran",
    "f03" => "gfortran",
    "f08" => "gfortran",

    // Template engines and preprocessors
    "pug" => "node",
    "hbs" => "node",
    "ejs" => "node",
    "njk" => "node",

    // Emerging or less common but widely used
    "dart" => "dart",
    "nim" => "nim",
    "zig" => "zig",
    "cr" => "crystal",
    "groovy" => "groovy"
};

const COMPILED_FILE_TYPES: [&str; 32] = [
    // General-purpose compiled languages
    "rs", "c", "cpp", "swift", "go", "kt", "kts", "cs", "vb", "fs", "scala", "dart",
    // Functional programming languages (compiled)
    "hs", "ml", // Low-level and systems programming
    "asm", "s", "zig", "nim", "cr", // Hardware description languages
    "v", "sv", "vhdl", "ada", // Scientific/numerical computing
    "f90", "f95", "f03", "f08",
    // Web technologies that require compilation or preprocessing
    "jsx", "tsx", "astro", "svelte", "vue",
];

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

pub fn try_compile(input_dir: &str) -> Result<()> {
    let path = Path::new(input_dir);
    if path.exists() {
        walk_dir(path, test_callback).context("failed to walk directory")?;
        match FILETYPES.lock() {
            std::result::Result::Ok(file_types) => {}
            Err(e) => {}
        }

        println!("File types collected.");
        Ok(())
    } else {
        Err(CompileError::InvalidPath(input_dir.to_string()).into())
    }
}

fn walk_dir(input_dir: &Path, callback: fn(&DirEntry)) -> Result<()> {
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

// async fn walk_dir_async(input_dir: &Path) -> Result<()> {
//     if !input_dir.is_dir() {
//         return Err(anyhow!("Input directory was now valid"))
//     }

//     let entries = tokio::fs::read_dir(input_dir).await?;

//     let mut dir_entries: Vec<&str>  = vec![];
    

    
//     Ok(())
// }

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
