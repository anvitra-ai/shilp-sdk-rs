use crate::client::Client;
use crate::error::Result;
use crate::models::{
    DebugDistanceResponse, DebugLevelsResponse, DebugNodeInfoResponse,
    DebugNodesAtLevelResponse, DebugReferenceNodeResponse,
};
use std::collections::HashMap;

impl Client {
    /// Gets the distance of a node in a collection for debug purposes
    pub async fn get_collection_distance(
        &self,
        collection_name: &str,
        field: &str,
        node_id: i32,
        text: &str,
    ) -> Result<DebugDistanceResponse> {
        let path = format!(
            "/api/collections/v1/debug/{}/{}/distance/{}",
            collection_name, field, node_id
        );
        let mut params = HashMap::new();
        params.insert("text".to_string(), text.to_string());

        self.do_request::<DebugDistanceResponse, ()>(
            reqwest::Method::GET,
            &path,
            None,
            Some(&params),
        )
        .await
    }

    /// Gets node info of a collection for debug purposes
    pub async fn get_collection_node_info(
        &self,
        collection_name: &str,
        field: &str,
        node_id: i32,
    ) -> Result<DebugNodeInfoResponse> {
        let path = format!(
            "/api/collections/v1/debug/{}/{}/nodes/{}",
            collection_name, field, node_id
        );
        self.do_request::<DebugNodeInfoResponse, ()>(reqwest::Method::GET, &path, None, None)
            .await
    }

    /// Gets node neighbors at a level of a collection for debug purposes
    pub async fn get_collection_node_neighbors_at_level(
        &self,
        collection_name: &str,
        field: &str,
        node_id: i32,
        level: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<DebugNodeInfoResponse> {
        let path = format!(
            "/api/collections/v1/debug/{}/{}/nodes/{}/neighbors/{}",
            collection_name, field, node_id, level
        );

        let mut params = HashMap::new();
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }

        self.do_request::<DebugNodeInfoResponse, ()>(
            reqwest::Method::GET,
            &path,
            None,
            if params.is_empty() {
                None
            } else {
                Some(&params)
            },
        )
        .await
    }

    /// Gets levels of a collection for debug purposes
    pub async fn get_collection_levels(
        &self,
        collection_name: &str,
    ) -> Result<DebugLevelsResponse> {
        let path = format!("/api/collections/v1/debug/{}/levels", collection_name);
        self.do_request::<DebugLevelsResponse, ()>(reqwest::Method::GET, &path, None, None)
            .await
    }

    /// Gets nodes at a level of a collection for debug purposes
    pub async fn get_collection_nodes_at_level(
        &self,
        collection_name: &str,
        level: i32,
    ) -> Result<DebugNodesAtLevelResponse> {
        let path = format!(
            "/api/collections/v1/debug/{}/levels/{}",
            collection_name, level
        );
        self.do_request::<DebugNodesAtLevelResponse, ()>(reqwest::Method::GET, &path, None, None)
            .await
    }

    /// Gets node by reference node ID of a collection for debug purposes
    pub async fn get_collection_node_by_reference_node_id(
        &self,
        collection_name: &str,
        node_id: i32,
    ) -> Result<DebugReferenceNodeResponse> {
        let path = format!(
            "/api/collections/v1/debug/{}/nodes/reference_node/{}",
            collection_name, node_id
        );
        self.do_request::<DebugReferenceNodeResponse, ()>(reqwest::Method::GET, &path, None, None)
            .await
    }
}
