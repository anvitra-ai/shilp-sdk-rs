use crate::client::Client;
use crate::error::{Result, ShilpError};
use crate::models::{
    FileReaderOptions, IngestRequest, IngestResponse, IngestSourceType,
    ListEmbeddingModelsResponse, ListIngestionSourcesResponse, ListNLIVerticalsResponse,
    ListStorageResponse, ReadDocumentResponse, SearchRequest, SearchResponse,
};
use std::collections::HashMap;

impl Client {
    /// Ingests data into a collection
    pub async fn ingest_data(&self, req: &IngestRequest) -> Result<IngestResponse> {
        self.do_request(
            reqwest::Method::POST,
            "/api/data/v1/ingest",
            Some(req),
            None,
        )
        .await
    }

    /// Searches for data in a collection using POST request
    /// This method supports field-specific weights via the SearchRequest.weights field
    pub async fn search_data(&self, req: &SearchRequest) -> Result<SearchResponse> {
        if req.collection.is_empty() {
            return Err(ShilpError::ValidationError(
                "collection name cannot be empty".to_string(),
            ));
        }
        if req.vector_query.is_none() && req.query.as_ref().map_or(true, |q| q.is_empty()) {
            return Err(ShilpError::ValidationError(
                "both vector_query and query cannot be empty".to_string(),
            ));
        }

        self.do_request(
            reqwest::Method::POST,
            "/api/data/v1/search",
            Some(req),
            None,
        )
        .await
    }

    /// Lists contents of a directory in uploads storage
    /// If the source is mongodb, then empty path lists all DBs. If path is a DB, lists all collections in that DB.
    pub async fn list_storage(
        &self,
        path: Option<&str>,
        _source: IngestSourceType,
    ) -> Result<ListStorageResponse> {
        let mut params = HashMap::new();
        if let Some(p) = path {
            params.insert("path".to_string(), p.to_string());
        }

        self.do_request::<ListStorageResponse, ()>(
            reqwest::Method::GET,
            "/api/data/v1/storage/list",
            None,
            Some(&params),
        )
        .await
    }

    /// Lists available ingestion sources
    pub async fn list_ingest_sources(&self) -> Result<ListIngestionSourcesResponse> {
        self.do_request::<ListIngestionSourcesResponse, ()>(
            reqwest::Method::GET,
            "/api/data/v1/ingest/sources",
            None,
            None,
        )
        .await
    }

    /// Reads the first few rows of a CSV document or MongoDB collection
    /// If the source is mongodb, then path is in the format "database/collection"
    /// options.query can be used to filter the documents returned in case of mongodb
    pub async fn read_document(
        &self,
        path: &str,
        options: &FileReaderOptions,
    ) -> Result<ReadDocumentResponse> {
        if path.is_empty() {
            return Err(ShilpError::ValidationError(
                "path cannot be empty".to_string(),
            ));
        }

        if let Some(rows) = options.limit {
            if rows < 0 {
                return Err(ShilpError::ValidationError(
                    "rows cannot be negative".to_string(),
                ));
            }
        }

        if let Some(skip) = options.skip {
            if skip < 0 {
                return Err(ShilpError::ValidationError(
                    "skip cannot be negative".to_string(),
                ));
            }
        }

        if let Some(ref source) = options.source {
            if *source == IngestSourceType::MongoDB && path.split('/').count() != 2 {
                return Err(ShilpError::ValidationError(
                    "for mongodb source, path must be in the format 'database/collection'"
                        .to_string(),
                ));
            }

            if !source.is_valid() {
                return Err(ShilpError::ValidationError(format!(
                    "invalid source type - {:?}",
                    source
                )));
            }
        }

        let mut params = HashMap::new();
        params.insert("path".to_string(), path.to_string());

        if let Some(ref source) = options.source {
            params.insert("source".to_string(), format!("{:?}", source).to_lowercase());
        }

        if let Some(rows) = options.limit {
            params.insert("rows".to_string(), rows.to_string());
        }

        if let Some(skip) = options.skip {
            params.insert("skip".to_string(), skip.to_string());
        }

        if let Some(ref mongo_filter) = options.mongo_filter {
            let filter_str = serde_json::to_string(mongo_filter)?;
            params.insert("mongo_filter".to_string(), filter_str);
        }

        self.do_request::<ReadDocumentResponse, ()>(
            reqwest::Method::GET,
            "/api/data/v1/storage/read",
            None,
            Some(&params),
        )
        .await
    }

    /// Uploads a data file to the uploads storage which can be used for ingestion
    pub async fn upload_data_file(&self, file_path: &std::path::Path) -> Result<()> {
        self.do_file_request(
            reqwest::Method::POST,
            "/api/data/v1/storage/upload",
            file_path,
        )
        .await
    }

    /// Lists all available embedding providers and their models
    pub async fn list_embedding_models(&self) -> Result<ListEmbeddingModelsResponse> {
        self.do_request::<ListEmbeddingModelsResponse, ()>(
            reqwest::Method::GET,
            "/api/data/v1/embedding/models",
            None,
            None,
        )
        .await
    }

    /// Lists all available NLI verticals
    pub async fn list_nli_verticals(&self) -> Result<ListNLIVerticalsResponse> {
        self.do_request::<ListNLIVerticalsResponse, ()>(
            reqwest::Method::GET,
            "/api/data/v1/nli/verticals",
            None,
            None,
        )
        .await
    }
}
