use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Generic response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericResponse {
    pub success: bool,
    pub message: String,
}

// Storage backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StorageBackendType {
    DoesNotExist = -1,
    File = 0,
    S3 = 1,
}

impl<'de> Deserialize<'de> for StorageBackendType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            -1 => Ok(StorageBackendType::DoesNotExist),
            1 => Ok(StorageBackendType::File),
            2 => Ok(StorageBackendType::S3),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid storage backend type: {}",
                value
            ))),
        }
    }
}

impl Serialize for StorageBackendType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl StorageBackendType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageBackendType::File => "disk",
            StorageBackendType::S3 => "s3",
            StorageBackendType::DoesNotExist => "unknown",
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, StorageBackendType::File | StorageBackendType::S3)
    }
}

// Attribute types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AttrType {
    Int64 = 0,
    Float64 = 1,
    String = 2,
    Bool = 3,
}

impl AttrType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AttrType::Int64 => "int64",
            AttrType::Float64 => "float64",
            AttrType::String => "string",
            AttrType::Bool => "bool",
        }
    }
}

impl<'de> Deserialize<'de> for AttrType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(AttrType::Int64),
            1 => Ok(AttrType::Float64),
            2 => Ok(AttrType::String),
            3 => Ok(AttrType::Bool),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid attribute type: {}",
                value
            ))),
        }
    }
}

impl Serialize for AttrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

// Metadata column schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataColumnSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub attr_type: AttrType,
}

// Collection structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub is_loaded: bool,
    pub fields: Option<Vec<String>>,
    pub searchable_fields: Option<Vec<String>>,
    #[serde(default)]
    pub metadata: Option<Vec<MetadataColumnSchema>>,
    pub has_metadata_enabled: bool,
    pub no_reference_storage: bool,
    pub storage_type: StorageBackendType,
    pub reference_storage_type: StorageBackendType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pq_enabled: Option<bool>,
}

// Metadata support info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataSupportInfo {
    pub support_metadata: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub storage_type: StorageBackendType,
    pub is_default: bool,
}

// List collections response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionsResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<Collection>,
    pub metadata_info: Vec<MetadataSupportInfo>,
}

// Add collection request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCollectionRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_reference_storage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_metadata_storage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageBackendType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_storage_type: Option<StorageBackendType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pq: Option<bool>,
}

// Record data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordData {
    pub id: String,
    pub expiry: Option<i64>,
    pub fields: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_fields: Option<HashMap<String, i32>>,
}

// Insert record request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertRecordRequest {
    pub collection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub record: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_fields: Option<HashMap<String, AttrType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectors: Option<HashMap<String, Vec<f32>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

// Insert record response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertRecordResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_records: Option<i32>,
}

// Ingest source type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IngestSourceType {
    File,
    #[serde(rename = "mongodb")]
    MongoDB,
}

impl IngestSourceType {
    pub fn is_valid(&self) -> bool {
        matches!(self, IngestSourceType::File | IngestSourceType::MongoDB)
    }
}

// Ingest request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<IngestSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_fetch_batch_size: Option<i32>,
    pub collection_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_fields: Option<HashMap<String, AttrType>>,
    pub fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_batch_size: Option<i32>,
}

// Ingest response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

// List ingestion sources response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIngestionSourcesResponse {
    pub message: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<IngestSourceType>>,
}

// File reader options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileReaderOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<IngestSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_filter: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

// Filter operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum FilterOp {
    #[serde(rename = "0")]
    Equals = 0,
    #[serde(rename = "1")]
    NotEquals = 1,
    #[serde(rename = "2")]
    GreaterThan = 2,
    #[serde(rename = "3")]
    GreaterThanOrEqual = 3,
    #[serde(rename = "4")]
    LessThan = 4,
    #[serde(rename = "5")]
    LessThanOrEqual = 5,
    #[serde(rename = "6")]
    In = 6,
    #[serde(rename = "7")]
    NotIn = 7,
}

impl FilterOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            FilterOp::Equals => "=",
            FilterOp::NotEquals => "!=",
            FilterOp::GreaterThan => ">",
            FilterOp::GreaterThanOrEqual => ">=",
            FilterOp::LessThan => "<",
            FilterOp::LessThanOrEqual => "<=",
            FilterOp::In => "IN",
            FilterOp::NotIn => "NOT IN",
        }
    }
}

// Filter expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<FilterOp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<serde_json::Value>>,
}

// Compound filter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompoundFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<FilterExpression>>,
}

// Sort order
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i32)]
pub enum SortOrder {
    #[serde(rename = "0")]
    Ascending = 0,
    #[serde(rename = "1")]
    Descending = 1,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        }
    }
}

// Sort expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortExpression {
    pub attribute: String,
    pub order: SortOrder,
}

// Compound sort
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompoundSort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<SortExpression>>,
}

// Search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub collection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CompoundFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CompoundSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_query: Option<Vec<f32>>,
}

// Search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub success: bool,
    pub message: Option<String>,
    pub data: Vec<HashMap<String, serde_json::Value>>,
}

// Storage item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    pub name: String,
    #[serde(rename = "isDir")]
    pub is_dir: bool,
}

// Storage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageData {
    pub items: Vec<StorageItem>,
}

// List storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStorageResponse {
    pub success: bool,
    pub message: String,
    pub data: StorageData,
}

// Read document response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadDocumentResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<HashMap<String, String>>,
}

// Health response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub success: bool,
    pub version: String,
}

// Debug distance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugDistanceData {
    pub distance: f64,
    pub vector: Vec<f64>,
}

// Debug distance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugDistanceResponse {
    pub success: bool,
    pub message: String,
    pub data: DebugDistanceData,
}

// Debug neighbor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugNeighbor {
    pub node_id: i32,
    pub vector_id: String,
    pub field: String,
    pub distance: f64,
    pub metadata: HashMap<String, serde_json::Value>,
}

// Debug node info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugNodeInfo {
    pub node_id: i32,
    pub vector_id: String,
    pub field: String,
    pub level: i32,
    pub metadata: HashMap<String, serde_json::Value>,
    pub neighbors: Vec<DebugNeighbor>,
}

// Debug node info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugNodeInfoResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DebugNodeInfo>,
}

// Debug level info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLevelInfo {
    pub level: i32,
    pub node_count: i32,
}

// Debug levels response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLevelsResponse {
    pub success: bool,
    pub message: String,
    pub data: HashMap<String, Vec<DebugLevelInfo>>,
}

// Debug nodes at level response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugNodesAtLevelResponse {
    pub success: bool,
    pub message: String,
    pub data: HashMap<String, Vec<i32>>,
}

// Debug vector node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugVectorNode {
    pub id: i32,
    pub field: String,
    pub vector: Vec<f64>,
}

// Debug reference node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugReferenceNode {
    pub id: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub nodes: Vec<DebugVectorNode>,
}

// Debug reference node response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugReferenceNodeResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DebugReferenceNode>,
}

// Embedding model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModel {
    pub name: String,
    pub is_default: bool,
}

// Embedding provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingProvider {
    pub name: String,
    pub is_default: bool,
    pub models: Vec<EmbeddingModel>,
}

// List embedding models response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEmbeddingModelsResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<EmbeddingProvider>,
    pub supports_distributed_embedding: bool,
}

// Oplog operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OpType {
    Insert,
    Update,
    Delete,
    #[serde(rename = "drop_collection")]
    DropCollection,
    #[serde(rename = "rename_collection")]
    RenameCollection,
}

// Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub fields: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_fields: Option<HashMap<String, AttrType>>,
    #[serde(skip)]
    pub vectors: Option<HashMap<String, Vec<f32>>>,
    #[serde(skip)]
    pub dist: Option<f32>,
    #[serde(skip)]
    pub nodes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
}

// Oplog entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OplogEntry {
    pub lsn: u64,
    pub timestamp: String,
    pub collection: String,
    pub doc_id: String,
    pub op_type: OpType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_doc: Option<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectors: Option<HashMap<String, Vec<f32>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_fields: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_fields: Option<HashMap<String, AttrType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

// Oplog status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OplogStatusResponse {
    pub success: bool,
    pub message: String,
    pub last_lsn: u64,
    pub retention_lsn: u64,
    pub replica_count: i32,
}

// Update replica LSN request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReplicaLSNRequest {
    pub collection: String,
    pub replica_id: String,
    pub lsn: u64,
}

// Update replica LSN response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReplicaLSNResponse {
    pub success: bool,
    pub message: String,
}

// Register replica request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterReplicaRequest {
    pub replica_id: String,
}

// Unregister replica request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnRegisterReplicaRequest {
    pub replica_id: String,
}

// Get oplog response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOplogResponse {
    pub success: bool,
    pub message: String,
    pub entries: Vec<OplogEntry>,
    pub last_lsn: u64,
    pub count: i32,
}

// Replica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replica {
    pub id: String,
    pub address: String,
    pub is_healthy: bool,
    pub is_syncing: bool,
}

// Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub write_replica: Replica,
    pub read_replicas: Vec<Replica>,
    pub available_count: i32,
    pub total_count: i32,
}

// Proxy stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStats {
    pub active_proxies: i32,
    pub targets: Vec<String>,
}

// Discovery stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStats {
    pub registry: Status,
    pub proxy: ProxyStats,
}

// Sync status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    Ready,
    Syncing,
}

// Update sync status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSyncStatusRequest {
    pub account_id: String,
    pub address: String,
    pub status: SyncStatus,
}

// Register to discovery request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterToDiscoveryRequest {
    pub account_id: String,
    pub address: String,
    pub id: String,
    pub is_read: bool,
    pub is_write: bool,
}

// Replica type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaType {
    Read,
    Write,
    SingleNode,
}

impl ReplicaType {
    pub fn is_read(&self) -> bool {
        matches!(self, ReplicaType::Read)
    }

    pub fn is_write(&self) -> bool {
        matches!(self, ReplicaType::Write)
    }

    pub fn is_single_node(&self) -> bool {
        matches!(self, ReplicaType::SingleNode)
    }
}
