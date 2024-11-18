// omniforge-state/src/main.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::{Connection, params, OptionalExtension};
use axum::{
    routing::{get, post, delete},
    extract::{Path, State, Json},
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, Deserialize};
use tracing::{info, error, Level};
use chrono::{DateTime, Utc};

/// Represents a key-value pair in the state store
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyValue {
    key: String,
    value: Vec<u8>,
    version: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: DateTime<Utc>,
}

/// Response structure for API endpoints
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Custom error type for the state service
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for StateError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            StateError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            StateError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
            StateError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(message),
        });

        (status, body).into_response()
    }
}

/// StateManager handles all interactions with the SQLite database
#[derive(Clone)]
struct StateManager {
    conn: Arc<Mutex<Connection>>,
}

impl StateManager {
    /// Create a new StateManager instance
    fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        // Open SQLite connection
        let conn = Connection::open(db_path)?;

        // Initialize database schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS state (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL,
                version INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_state_updated_at ON state(updated_at)",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get a value from the store
    async fn get(&self, key: &str) -> Result<Option<KeyValue>, StateError> {
        let conn = self.conn.lock().await;
        let result = conn.query_row(
            "SELECT key, value, version, created_at, updated_at FROM state WHERE key = ?",
            params![key],
            |row| {
                Ok(KeyValue {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    version: row.get(2)?,
                    created_at: DateTime::from_timestamp(row.get(3)?, 0)
                        .ok_or_else(|| rusqlite::Error::InvalidParameterName("Invalid timestamp".into()))?,
                    updated_at: DateTime::from_timestamp(row.get(4)?, 0)
                        .ok_or_else(|| rusqlite::Error::InvalidParameterName("Invalid timestamp".into()))?,
                })
            },
        ).optional()?;

        Ok(result)
    }

    /// Put a value into the store
    async fn put(&self, key: String, value: Vec<u8>) -> Result<KeyValue, StateError> {
        let conn = self.conn.lock().await;
        let now = Utc::now();
        let timestamp = now.timestamp();

        // Get current version if exists
        let current_version: i64 = conn
            .query_row(
                "SELECT version FROM state WHERE key = ?",
                params![key],
                |row| row.get(0),
            )
            .optional()?
            .unwrap_or(0);

        let new_version = current_version + 1;

        conn.execute(
            "INSERT INTO state (key, value, version, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                version = excluded.version,
                updated_at = excluded.updated_at",
            params![key, value, new_version, timestamp, timestamp],
        )?;

        Ok(KeyValue {
            key,
            value,
            version: new_version,
            created_at: now,
            updated_at: now,
        })
    }

    /// Delete a value from the store
    async fn delete(&self, key: &str) -> Result<bool, StateError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute("DELETE FROM state WHERE key = ?", params![key])?;
        Ok(rows > 0)
    }

    /// List all keys
    async fn list(&self) -> Result<Vec<String>, StateError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT key FROM state")?;
        let keys = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(keys)
    }
}

async fn get_value(
    Path(key): Path<String>,
    State(state_manager): State<StateManager>,
) -> Result<Json<ApiResponse<KeyValue>>, StateError> {
    match state_manager.get(&key).await? {
        Some(value) => Ok(Json(ApiResponse {
            success: true,
            data: Some(value),
            error: None,
        })),
        None => Err(StateError::NotFound(key)),
    }
}

async fn put_value(
    Path(key): Path<String>,
    State(state_manager): State<StateManager>,
    Json(value): Json<Vec<u8>>,
) -> Result<Json<ApiResponse<KeyValue>>, StateError> {
    let kv = state_manager.put(key, value).await?;
    Ok(Json(ApiResponse {
        success: true,
        data: Some(kv),
        error: None,
    }))
}

async fn delete_value(
    Path(key): Path<String>,
    State(state_manager): State<StateManager>,
) -> Result<Json<ApiResponse<bool>>, StateError> {
    let deleted = state_manager.delete(&key).await?;
    Ok(Json(ApiResponse {
        success: true,
        data: Some(deleted),
        error: None,
    }))
}

async fn list_keys(
    State(state_manager): State<StateManager>,
) -> Result<Json<ApiResponse<Vec<String>>>, StateError> {
    let keys = state_manager.list().await?;
    Ok(Json(ApiResponse {
        success: true,
        data: Some(keys),
        error: None,
    }))
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

#[tokio::main]
pub async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Create state manager
    let state_manager = StateManager::new("omniforge_state.db").expect("Failed to create state manager");

    // Create router
    let app = Router::new()
        .route("/v1/state/:key", get(get_value))
        .route("/v1/state/:key", post(put_value))
        .route("/v1/state/:key", delete(delete_value))
        .route("/v1/state", get(list_keys))
        .route("/health", get(health))
        .with_state(state_manager);

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Starting state service on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}