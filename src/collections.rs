use crate::client::Client;
use crate::error::Result;
use crate::models::{
    AddCollectionRequest, GenericResponse, InsertRecordRequest, InsertRecordResponse,
    ListCollectionsResponse,
};
use reqwest::Response;

impl Client {
    /// Lists all collections
    pub async fn list_collections(&self) -> Result<ListCollectionsResponse> {
        self.do_request::<ListCollectionsResponse, ()>(
            reqwest::Method::GET,
            "/api/collections/v1/",
            None,
            None,
        )
        .await
    }

    /// Adds a new collection
    pub async fn add_collection(&self, req: &AddCollectionRequest) -> Result<GenericResponse> {
        self.do_request(
            reqwest::Method::POST,
            "/api/collections/v1/",
            Some(req),
            None,
        )
        .await
    }

    /// Deletes a record from a collection
    pub async fn delete_record(
        &self,
        collection_name: &str,
        id: &str,
    ) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/{}", collection_name, id);
        self.do_request::<GenericResponse, ()>(reqwest::Method::DELETE, &path, None, None)
            .await
    }

    /// Performs expiry cleanup on a collection
    pub async fn expiry_cleanup(&self, collection_name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/expiry-cleanup", collection_name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::POST, &path, None, None)
            .await
    }

    /// Drops an existing collection
    pub async fn drop_collection(&self, name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}", name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::DELETE, &path, None, None)
            .await
    }

    /// Flushes a collection to disk
    pub async fn flush_collection(&self, name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/flush", name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::POST, &path, None, None)
            .await
    }

    /// Loads a collection into memory
    pub async fn load_collection(&self, name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/load", name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::POST, &path, None, None)
            .await
    }

    /// Unloads a collection from memory
    pub async fn unload_collection(&self, name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/unload", name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::POST, &path, None, None)
            .await
    }

    /// Exports a collection and returns a Response for downloading the file
    /// The caller is responsible for processing the response (e.g., saving to a file)
    pub async fn export_collection(&self, name: &str) -> Result<Response> {
        let path = format!("/api/collections/v1/{}/export", name);
        self.do_request_with_file_response(reqwest::Method::POST, &path, None)
            .await
    }

    /// Imports a collection from a file
    pub async fn import_collection(&self, file_path: &std::path::Path) -> Result<()> {
        self.do_file_request(
            reqwest::Method::POST,
            "/api/collections/v1/import",
            file_path,
        )
        .await
    }

    /// Renames an existing collection
    pub async fn rename_collection(
        &self,
        old_name: &str,
        new_name: &str,
    ) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/rename/{}", old_name, new_name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::PUT, &path, None, None)
            .await
    }

    /// Re-indexes a collection for debug purposes
    pub async fn reindex_collection(&self, collection_name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/reindex", collection_name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::PUT, &path, None, None)
            .await
    }

    /// Performs Product Quantization training for an existing collection
    pub async fn pq_train(&self, collection_name: &str) -> Result<GenericResponse> {
        let path = format!("/api/collections/v1/{}/pq-train", collection_name);
        self.do_request::<GenericResponse, ()>(reqwest::Method::POST, &path, None, None)
            .await
    }

    /// Inserts a new record into a collection
    pub async fn insert_record(&self, req: &InsertRecordRequest) -> Result<InsertRecordResponse> {
        self.do_request(
            reqwest::Method::POST,
            "/api/collections/v1/record",
            Some(req),
            None,
        )
        .await
    }
}
