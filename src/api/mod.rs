use tokio::fs;
use warp::{Filter, Rejection, Reply};
use hyper::body::Buf;
use std::fs::File;
use std::io::Write;
use tar::Archive;
use futures_util::stream::TryStreamExt;
use std::path::Path;
use tracing::{info, error, warn, Level};
use tracing_subscriber;

#[derive(Debug)]
pub enum UploadError {
    IoError(std::io::Error),
    MultipartError(warp::Error),
    ValidationError(String),
}

impl warp::reject::Reject for UploadError {}

async fn upload_tarball(mut form: warp::multipart::FormData) -> Result<impl Reply, Rejection> {
    // Ensure upload directory exists
    fs::create_dir_all("./App").await.map_err(|e| {
        eprintln!("Failed to create directory: {}", e);
        warp::reject::reject()
    })?;

    while let Some(part) = form.try_next().await.map_err(|e| {
        eprintln!("Form error: {}", e);
        warp::reject::reject()
    })? {
        if part.name() == "file" {
            // Get filename or use default
            let filename = part.filename()
                .unwrap_or("upload.tar.gz")
                .to_string();
            
            let filepath = format!("./App/{}", filename);
            
            // Stream the file contents into a buffer
            let mut buffer = vec![];
            let mut stream = part.stream();
            
            while let Some(chunk) = stream.try_next().await.map_err(|e| {
                eprintln!("Stream error: {}", e);
                warp::reject::reject()
            })? {
                buffer.extend_from_slice(chunk.chunk());
            }

            // Write the buffer to a file
            tokio::fs::write(&filepath, &buffer).await.map_err(|e| {
                eprintln!("File write error: {}", e);
                warp::reject::reject()
            })?;

            // Open and extract the tar.gz
            let tar_gz = std::fs::File::open(&filepath).map_err(|e| {
                eprintln!("Failed to open tar.gz: {}", e);
                warp::reject::reject()
            })?;
            
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);
            
            archive.unpack("./App").map_err(|e| {
                eprintln!("Failed to extract archive: {}", e);
                warp::reject::reject()
            })?;

            // Clean up the tar.gz file
            fs::remove_file(&filepath).await.map_err(|e| {
                eprintln!("Cleanup error: {}", e);
                warp::reject::reject()
            })?;

            return Ok(warp::reply::with_status(
                "File uploaded and extracted successfully",
                warp::http::StatusCode::OK,
            ));
        }
    }

    Err(warp::reject::reject())
}


async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(upload_error) = err.find::<UploadError>() {
        let (message, status) = match upload_error {
            UploadError::IoError(e) => {
                error!("IO Error during upload: {}", e);
                (format!("IO Error: {}", e), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            UploadError::MultipartError(e) => {
                error!("Multipart Error during upload: {}", e);
                (format!("Upload Error: {}", e), warp::http::StatusCode::BAD_REQUEST)
            }
            UploadError::ValidationError(msg) => {
                error!("Validation Error during upload: {}", msg);
                (msg.clone(), warp::http::StatusCode::BAD_REQUEST)
            }
        };
        
        Ok(warp::reply::with_status(message, status))
    } else {
        error!("Unhandled rejection: {:?}", err);
        Err(err)
    }
}

pub async fn listen() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Initializing upload server");

    // Create the upload route
    let upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(10_000_000))
        .and_then(upload_tarball);

    let routes = upload_route.recover(handle_rejection);

    info!("Server starting on http://127.0.0.1:3030");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}