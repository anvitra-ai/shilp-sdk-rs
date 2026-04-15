use shilp_sdk::{
    models::{AddCollectionRequest, InsertRecordRequest, SearchRequest},
    Client,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // Initialize the client
    let client = Client::new("http://localhost:3000");

    // Check health
    let health = client.health_check().await?;
    println!("Health: {}", health.success);
    println!("Version: {}", health.version);

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
        storage_type: Some(shilp_sdk::models::StorageBackendType::File),
        reference_storage_type: Some(shilp_sdk::models::StorageBackendType::File),
        enable_pq: None,
    };
    let add_result = client.add_collection(&req).await?;
    println!("Collection added: {}", add_result.success);

    // Insert a record
    let mut record = HashMap::new();
    record.insert("title".to_string(), serde_json::json!("Hello World"));

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
        vector_config: None,
        array_fields: None,
    };
    let insert_result = client.insert_record(&insert_req).await?;
    println!("Record inserted: {}", insert_result.success);

    // Flush collection (required after insert_record)
    let flush_result = client.flush_collection("my-collection").await?;
    println!("Collection flushed: {}", flush_result.success);

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
        use_nli: None,
        field_config: None,
        queries: None,
        vector_queries: None,
        fuzzy_algo: None,
    };
    let results = client.search_data(&search_req).await?;
    println!("Search results: {:?}", results.data);

    // Advanced search with max distance filter
    let max_dist = 0.5;
    let advanced_search_req = SearchRequest {
        collection: "my-collection".to_string(),
        query: Some("Hello".to_string()),
        fields: Some(vec!["title".to_string()]),
        limit: Some(10),
        max_distance: Some(max_dist),
        weights: None,
        filters: None,
        sort: None,
        vector_query: None,
        use_nli: None,
        field_config: None,
        queries: None,
        vector_queries: None,
        fuzzy_algo: None,
    };
    let advanced_results = client.search_data(&advanced_search_req).await?;
    println!("Advanced search results: {:?}", advanced_results.data);

    // Drop collection
    let drop_result = client.drop_collection("my-collection").await?;
    println!("Collection dropped: {}", drop_result.success);

    Ok(())
}
