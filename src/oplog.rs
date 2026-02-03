use crate::client::Client;
use crate::error::Result;
use crate::models::{
    GenericResponse, GetOplogResponse, OplogStatusResponse, RegisterReplicaRequest,
    UnRegisterReplicaRequest, UpdateReplicaLSNRequest, UpdateReplicaLSNResponse,
};
use std::collections::HashMap;

impl Client {
    /// Retrieves oplog entries after a specific LSN for replica synchronization
    /// Parameters:
    /// - collection: Collection name to retrieve oplog entries for (optional, if empty returns entries for all collections)
    /// - after_lsn: LSN after which to retrieve oplog entries (required)
    /// - limit: Maximum number of oplog entries to retrieve (optional)
    pub async fn get_oplog_entries(
        &self,
        collection: Option<&str>,
        after_lsn: u64,
        limit: Option<i32>,
    ) -> Result<GetOplogResponse> {
        let mut params = HashMap::new();
        params.insert("after_lsn".to_string(), after_lsn.to_string());

        if let Some(coll) = collection {
            params.insert("collection".to_string(), coll.to_string());
        }

        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }

        self.do_request::<GetOplogResponse, ()>(
            reqwest::Method::GET,
            "/api/oplog/v1/",
            None,
            Some(&params),
        )
        .await
    }

    /// Updates the last applied LSN for a replica (heartbeat)
    /// Parameters:
    /// - collection: Collection name
    /// - replica_id: Replica identifier
    /// - lsn: Last applied LSN
    pub async fn update_replica_lsn(
        &self,
        collection: &str,
        replica_id: &str,
        lsn: u64,
    ) -> Result<UpdateReplicaLSNResponse> {
        let req = UpdateReplicaLSNRequest {
            collection: collection.to_string(),
            replica_id: replica_id.to_string(),
            lsn,
        };

        self.do_request(
            reqwest::Method::POST,
            "/api/oplog/v1/heartbeat",
            Some(&req),
            None,
        )
        .await
    }

    /// Registers a replica for oplog retention tracking
    /// Parameters:
    /// - replica_id: Replica identifier
    pub async fn register_replica(&self, replica_id: &str) -> Result<GenericResponse> {
        let req = RegisterReplicaRequest {
            replica_id: replica_id.to_string(),
        };

        self.do_request(
            reqwest::Method::POST,
            "/api/oplog/v1/register",
            Some(&req),
            None,
        )
        .await
    }

    /// Unregisters a replica for oplog retention tracking
    /// Parameters:
    /// - replica_id: Replica identifier
    pub async fn unregister_replica(&self, replica_id: &str) -> Result<GenericResponse> {
        let req = UnRegisterReplicaRequest {
            replica_id: replica_id.to_string(),
        };

        self.do_request(
            reqwest::Method::POST,
            "/api/oplog/v1/unregister",
            Some(&req),
            None,
        )
        .await
    }

    /// Retrieves current oplog status and statistics for a collection
    /// Parameters:
    /// - collection: Collection name to retrieve oplog status for (required)
    pub async fn get_oplog_status(&self, collection: &str) -> Result<OplogStatusResponse> {
        let mut params = HashMap::new();
        params.insert("collection".to_string(), collection.to_string());

        self.do_request::<OplogStatusResponse, ()>(
            reqwest::Method::GET,
            "/api/oplog/v1/status",
            None,
            Some(&params),
        )
        .await
    }
}
