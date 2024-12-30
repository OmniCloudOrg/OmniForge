mod deploy;
use rocket::{http::{ContentType, Status}, post, Data};
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::{
    fs,
    io::{self, Cursor},
    path::PathBuf,
    str::FromStr,
};

#[post("/deploy", data = "<data>")]
pub async fn test(content_type: &ContentType, data: Data<'_>) -> Result<Status,Status> {
    println!("Starting deploy handler");
    println!("Content-Type: {:?}", content_type);

    let mut options = MultipartFormDataOptions::new();

    // Add multiple possible field names to help debug
    options
        .allowed_fields
        .push(MultipartFormDataField::file("media").size_limit(5 * 1024 * 1024 * 1024));
    options
        .allowed_fields
        .push(MultipartFormDataField::file("file").size_limit(5 * 1024 * 1024 * 1024));
    options
        .allowed_fields
        .push(MultipartFormDataField::file("upload").size_limit(5 * 1024 * 1024 * 1024));

    // Parse form data with detailed error handling
    let form_data = match MultipartFormData::parse(content_type, data, options).await {
        Ok(form) => {
            println!("Successfully parsed form data");
            form
        }
        Err(e) => {
            println!("Error parsing form data: {:?}", e);
            return Err(Status::new(400))
        }
    };

    // Print ALL available fields for debugging
    println!("Available fields in form_data:");
    println!("Raw fields: {:#?}", form_data.raw);
    println!("Text fields: {:#?}", form_data.texts);
    println!("Files: {:#?}", form_data.files);

    // Check each possible file field
    for field_name in ["media", "file", "upload"] {
        if let Some(files) = form_data.files.get(field_name) {
            println!("Found files in field '{}': {:?}", field_name, files);

            if let Some(file) = files.first() {
                println!("Processing file:");
                println!("  Path: {:?}", file.path);
                println!("  Filename: {:?}", file.file_name);
                println!("  Content-Type: {:?}", file.content_type);

                // Create App directory
                match fs::create_dir_all("./App") {
                    Ok(_) => {
                        let dir = std::path::PathBuf::from_str("./App").unwrap();
                        let canon_dir = dir.canonicalize().unwrap();
                        log::info!("Created Directory at {}",canon_dir.display())
                    },
                    Err(_) => {
                        return Err::<Status,Status>(Status::new(500));
                    },
                }
                    
            
                // Copy file with size verification
                let source_size = fs::metadata(&file.path)
                    .map_err(|_| return Err::<Status,Status>(Status::new(500))).unwrap()
                    .len();

                println!("Source file size: {} bytes", source_size);

                match fs::copy(&file.path, "./App/app.tar.gz") {
                    Ok(bytes_written) => {
                        println!("Successfully wrote {} bytes", bytes_written);
                        if bytes_written == source_size {
                            let file_path = PathBuf::from_str("./App/app.tar.gz")
                                .expect("Failed to get app zip");
                            let tar_gz = fs::File::open(&file_path).expect("Failed to open tar");
                            let tar = flate2::read::GzDecoder::new(tar_gz);
                            let mut archive = tar::Archive::new(tar);

                            archive.unpack("./App").unwrap();

                            // Clean up the tar.gz file
                            fs::remove_file(&file_path).expect("Fail");
                            return Ok(Status::new(200));
                                
                        } else {
                            return Err(Status::new(500))
                        }
                    }
                    Err(e) => {
                        println!("Error copying file: {:?}", e);
                        return Err(Status::new(500))
                    }
                }
            } else {
                println!("No valid file found in request");
                return Err(Status::new(500))
            }
        }
    }
    return Ok::<Status,Status>(Status::new(200));
}


// Helper function to verify tar file integrity
fn verify_tar_file(path: &str) -> io::Result<()> {
    println!("verifying file");
    use std::process::Command;

    let output = Command::new("tar").arg("-tzf").arg(path).output()?;

    if !output.status.success() {
        println!("Invalid tar data");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid tar.gz file",
        ));
    }

    Ok(())
}
