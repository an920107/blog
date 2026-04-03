use qdrant_client::{
    Qdrant,
    qdrant::{CollectionExistsRequest, CreateCollectionBuilder, Distance, VectorParamsBuilder},
};

#[derive(Clone)]
pub struct QdrantConfiguration {
    pub search_threshold: f32,
    pub search_limit: usize,
    pub post_collection_name: String,

    url: String,
    api_key: Option<String>,
    size: u64,
}

impl QdrantConfiguration {
    pub fn new() -> Self {
        let url =
            std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://127.0.0.1:6334".to_string());
        let api_key = std::env::var("QDRANT_API_KEY").ok();
        let search_threshold = std::env::var("QDRANT_SEARCH_THRESHOLD")
            .ok()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.8);
        let search_limit = std::env::var("QDRANT_SEARCH_LIMIT")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(15);

        let collection_name_prefix =
            std::env::var("QDRANT_COLLECTION_NAME_PREFIX").unwrap_or_else(|_| "blog".to_string());
        let post_collection_name = format!("{}_post", collection_name_prefix);

        Self {
            search_threshold,
            search_limit,
            post_collection_name,
            url,
            api_key,
            size: 1024,
        }
    }

    pub async fn create_client(&self) -> Qdrant {
        let client = Qdrant::from_url(&self.url)
            .api_key(self.api_key.clone())
            .build()
            .expect("Failed to create Qdrant client");

        let vector_config = VectorParamsBuilder::new(self.size, Distance::Cosine);

        if !is_collection_exists(&client, &self.post_collection_name).await {
            client
                .create_collection(
                    CreateCollectionBuilder::new(&self.post_collection_name)
                        .vectors_config(vector_config),
                )
                .await
                .expect("Failed to create Qdrant collection");
        }

        client
    }
}

async fn is_collection_exists(client: &Qdrant, collection_name: &str) -> bool {
    client
        .collection_exists(CollectionExistsRequest {
            collection_name: collection_name.to_string(),
        })
        .await
        .expect("Failed to check if Qdrant collection exists")
}
