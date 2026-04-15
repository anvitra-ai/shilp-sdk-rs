//! # Shilp Rust SDK
//!
//! This is the official Rust SDK for the Shilp Vector Database API.
//!
//! ## Features
//!
//! - Collection Management (List, Add, Drop, Rename, Load, Unload, Flush, ReIndex)
//! - Data Ingestion & Search (with keyword fields support)
//! - Record Management (Insert, Delete, Expiry Cleanup)
//! - Debug Collection Operations (Distance, Node Info, Levels, Neighbors)
//! - Oplog Operations (Replica Registration, Heartbeat, Get Entries, Status)
//! - Storage Listing
//! - Health Check
//! - Discovery Service Integration
//!
//! ## Example
//!
//! ```no_run
//! use shilp_sdk::{Client, models::AddCollectionRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the client
//!     let client = Client::new("http://localhost:3000");
//!
//!     // Check health
//!     let health = client.health_check().await?;
//!     println!("Health: {}", health.success);
//!
//!     // List collections
//!     let collections = client.list_collections().await?;
//!     println!("Collections: {:?}", collections.data);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod collections;
pub mod data;
pub mod debug;
pub mod discovery;
pub mod error;
pub mod health;
pub mod models;
pub mod oplog;
pub mod settings;

// Re-export commonly used types
pub use client::Client;
pub use discovery::DiscoveryClient;
pub use error::{Result, ShilpError};
