# Shilp Rust SDK

[![Crates.io](https://img.shields.io/crates/v/shilp-sdk.svg)](https://crates.io/crates/shilp-sdk)

This is the official Rust SDK for the Shilp Vector Database API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shilp-sdk = "0.12"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use shilp_sdk::{Client, models::{AddCollectionRequest, InsertRecordRequest, SearchRequest}};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = Client::new("http://localhost:3000");

    // Check health
    let health = client.health_check().await?;
    println!("Health: {}", health.success);

    // List collections
    let collections = client.list_collections().await?;
    println!("Collections: {:?}", collections.data);

    // Drop collection if exists
    let _ = client.drop_collection("my-collection").await;

    // Create a new collection
    let req = AddCollectionRequest {
        name: "my-collection".to_string(),
        no_reference_storage: None,
        has_metadata_storage: None,
        storage_type: None,
        reference_storage_type: None,
        enable_pq: None,
    };
    client.add_collection(&req).await?;

    // Insert a record
    let mut record = HashMap::new();
    record.insert("title".to_string(), serde_json::json!("Hello World"));
    record.insert("vector".to_string(), serde_json::json!([0.1, 0.2, 0.3]));

    let insert_req = InsertRecordRequest {
        collection: "my-collection".to_string(),
        id: Some("record-1".to_string()),
        record,
        expiry: None,
        metadata_fields: None,
        embedding_provider: None,
        fields: None,
        keyword_fields: None,
        vectors: None,
        model: None,
    };
    client.insert_record(&insert_req).await?;

    // Flush collection
    client.flush_collection("my-collection").await?;

    // Search
    let search_req = SearchRequest {
        collection: "my-collection".to_string(),
        query: Some("Hello".to_string()),
        fields: Some(vec!["title".to_string()]),
        limit: Some(10),
        weights: None,
        max_distance: None,
        filters: None,
        sort: None,
        vector_query: None,
    };
    let results = client.search_data(&search_req).await?;
    println!("Search results: {:?}", results.data);

    // Drop collection
    client.drop_collection("my-collection").await?;

    Ok(())
}
```

### Advanced Search with Filters

```rust
use shilp_sdk::models::{SearchRequest, CompoundFilter, FilterExpression, FilterOp};

let max_dist = 0.5;
let search_req = SearchRequest {
    collection: "my-collection".to_string(),
    query: Some("Hello".to_string()),
    fields: Some(vec!["title".to_string()]),
    limit: Some(10),
    max_distance: Some(max_dist),
    filters: None,
    sort: None,
    weights: None,
    vector_query: None,
};
let results = client.search_data(&search_req).await?;
```

### Debug Operations

The SDK provides debug endpoints for inspecting collection internals:

```rust
// Re-index a collection
client.reindex_collection("my-collection").await?;

// Get collection levels
let levels = client.get_collection_levels("my-collection").await?;

// Get nodes at a specific level
let nodes = client.get_collection_nodes_at_level("my-collection", 0).await?;

// Get node information
let node_info = client.get_collection_node_info("my-collection", "title", 123).await?;

// Get node neighbors at a level
let neighbors = client.get_collection_node_neighbors_at_level(
    "my-collection", "title", 123, 0, Some(10), Some(0)
).await?;

// Get distance to a node
let distance = client.get_collection_distance(
    "my-collection", "title", 123, "some text"
).await?;

// Get node by reference ID
let ref_node = client.get_collection_node_by_reference_node_id("my-collection", 456).await?;
```

### Oplog Operations

The SDK provides oplog (operation log) endpoints for replica synchronization:

```rust
// Register a replica for oplog tracking
let register_resp = client.register_replica("replica-1").await?;

// Get oplog status for a collection
let status = client.get_oplog_status("my-collection").await?;
println!("Oplog status - Last LSN: {}, Retention LSN: {}, Replicas: {}",
    status.last_lsn, status.retention_lsn, status.replica_count);

// Get oplog entries after a specific LSN
let entries = client.get_oplog_entries(Some("my-collection"), 1000, Some(100)).await?;
println!("Retrieved {} oplog entries, last LSN: {}", entries.count, entries.last_lsn);

// Update replica LSN (heartbeat)
let update_resp = client.update_replica_lsn("my-collection", "replica-1", 1050).await?;
println!("Updated replica LSN: {}", update_resp.success);
```

### Discovery Client

For service discovery and registration:

```rust
use shilp_sdk::{DiscoveryClient, models::ReplicaType};

let discovery = DiscoveryClient::new("http://discovery-server:8080");

// Register a Shilp service
discovery.register_shilp_service("account-1", "http://localhost:3000", "node-1", ReplicaType::SingleNode).await?;

// Get discovery stats
let stats = discovery.get_shilp_stats("account-1").await?;
```

## Features

- Collection Management (List, Add, Drop, Rename, Load, Unload, Flush, ReIndex)
- Data Ingestion & Search (with keyword fields support)
- Record Management (Insert, Delete, Expiry Cleanup)
- Debug Collection Operations (Distance, Node Info, Levels, Neighbors)
- Oplog Operations (Replica Registration, Heartbeat, Get Entries, Status)
- Storage Listing
- Health Check
- Discovery Service Integration

## License

MIT
