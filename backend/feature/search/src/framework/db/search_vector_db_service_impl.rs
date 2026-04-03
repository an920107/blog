use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use async_trait::async_trait;
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        Condition, DeletePointsBuilder, Filter, PointStruct, QueryPointGroupsBuilder,
        UpsertPointsBuilder, Value,
    },
};

use crate::{
    adapter::gateway::search_vector_db_service::SearchVectorDbService,
    domain::error::search_error::SearchError,
};

const KEY_POST_ID: &str = "post_id";
const KEY_CHUNK_INDEX: &str = "chunk_index";

pub struct SearchVectorDbServiceImpl {
    qdrant_client: Arc<Qdrant>,
    collection_name: String,
    search_threshold: f32,
    search_limit: usize,
}

impl SearchVectorDbServiceImpl {
    pub fn new(
        qdrant_client: Arc<Qdrant>,
        collection_name: &str,
        search_threshold: f32,
        search_limit: usize,
    ) -> Self {
        Self {
            qdrant_client,
            collection_name: collection_name.to_string(),
            search_threshold,
            search_limit,
        }
    }
}

#[async_trait]
impl SearchVectorDbService for SearchVectorDbServiceImpl {
    async fn insert_vectors(
        &self,
        post_id: i32,
        vectors: &Vec<Vec<f32>>,
    ) -> Result<(), SearchError> {
        let points: Vec<PointStruct> = vectors
            .into_iter()
            .enumerate()
            .map(|(index, vector)| {
                let payload = InnerPayload {
                    post_id,
                    chunk_index: index,
                };
                let id = payload.generate_vector_id();
                PointStruct::new(id, vector.clone(), payload)
            })
            .collect();

        let request = UpsertPointsBuilder::new(self.collection_name.clone(), points).wait(true);

        self.qdrant_client
            .upsert_points(request)
            .await
            .map_err(|e| SearchError::Unexpected(e.into()))?;

        Ok(())
    }

    async fn delete_vectors_by_post_id(&self, post_id: i32) -> Result<(), SearchError> {
        let filter = Filter::all([Condition::matches(KEY_POST_ID, post_id as i64)]);
        let request = DeletePointsBuilder::new(self.collection_name.clone())
            .points(filter)
            .wait(true);

        self.qdrant_client
            .delete_points(request)
            .await
            .map_err(|e| SearchError::Unexpected(e.into()))?;

        Ok(())
    }

    async fn search_similar_posts(
        &self,
        query_vector: &Vec<f32>,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, SearchError> {
        let filter = if let Some(scope) = scope {
            if scope.is_empty() {
                return Ok(vec![]);
            }
            Filter::all([Condition::matches(
                KEY_POST_ID,
                scope.iter().map(|id| *id as i64).collect::<Vec<_>>(),
            )])
        } else {
            Filter::all([])
        };

        let request = QueryPointGroupsBuilder::new(self.collection_name.clone(), KEY_POST_ID)
            .query(query_vector.clone())
            .with_payload(true)
            .filter(filter)
            .group_size(1u64)
            .score_threshold(self.search_threshold)
            .limit(self.search_limit as u64);

        let response = self
            .qdrant_client
            .query_groups(request)
            .await
            .map_err(|e| SearchError::Unexpected(e.into()))?;

        let groups = response
            .result
            .ok_or(SearchError::Unexpected(anyhow!(
                "No result groups returned from Qdrant"
            )))?
            .groups;

        groups
            .into_iter()
            .map(|group| {
                let payload = &group
                    .hits
                    .first()
                    .ok_or(SearchError::Unexpected(anyhow!(
                        "No hits returned in group"
                    )))?
                    .payload;

                match payload.get(KEY_POST_ID) {
                    Some(v) => match v.as_integer() {
                        Some(post_id) => Ok(post_id as i32),
                        _ => Err(SearchError::Unexpected(anyhow!(
                            "Invalid post_id type in payload"
                        ))),
                    },
                    _ => Err(SearchError::Unexpected(anyhow!(
                        "post_id not found in payload"
                    ))),
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct InnerPayload {
    post_id: i32,
    chunk_index: usize,
}

impl InnerPayload {
    fn generate_vector_id(&self) -> u64 {
        let high_bits = self.post_id as u64;
        let low_bits = self.chunk_index as u64;
        (high_bits << 32) | low_bits
    }
}

impl From<InnerPayload> for Payload {
    fn from(value: InnerPayload) -> Self {
        let map: HashMap<&str, Value> = HashMap::from([
            (KEY_POST_ID, (value.post_id as i64).into()),
            (KEY_CHUNK_INDEX, (value.chunk_index as i64).into()),
        ]);
        Self::from(map)
    }
}
