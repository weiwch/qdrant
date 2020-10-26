use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};
use segment::types::{PointIdType, PayloadKeyType};
use crate::operations::types::VectorType;
use std::collections::HashMap;
use crate::operations::payload_ops::PayloadInterface;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PointStruct {
    /// Point id
    pub id: PointIdType,
    /// Vector
    pub vector: VectorType,
    /// Payload values (optional)
    pub payload: Option<HashMap<PayloadKeyType, PayloadInterface>>,
}


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PointInsertOps {
    #[serde(rename = "batch")]
    /// Inset points from a batch.
    BatchPoints {
        ids: Vec<PointIdType>,
        vectors: Vec<VectorType>,
        payloads: Option<Vec<Option<HashMap<PayloadKeyType, PayloadInterface>>>>,
    },
    #[serde(rename = "points")]
    /// Insert points from a list
    PointsList(Vec<PointStruct>),
}


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PointOps {
    /// Insert or update points
    UpsertPoints(PointInsertOps),
    /// Delete point if exists
    DeletePoints {
        ids: Vec<PointIdType>,
    },
}
