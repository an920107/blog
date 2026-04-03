use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use fastembed::TextEmbedding;
use post::domain::entity::post::Post;
use text_splitter::MarkdownSplitter;
use tokenizers::Tokenizer;
use tokio::{sync::Mutex, task::spawn_blocking};

use crate::{
    adapter::gateway::search_vector_db_service::SearchVectorDbService,
    application::gateway::search_repository::SearchRepository,
    domain::error::search_error::SearchError,
};

// For details, please refer to https://huggingface.co/BAAI/bge-large-zh-v1.5
const QUERY_INSTRUCTION: &str = "为这个句子生成表示以用于检索相关文章：";

pub struct SearchRepositoryImpl {
    search_vector_db_service: Arc<dyn SearchVectorDbService>,
    markdown_splitter: Arc<MarkdownSplitter<Tokenizer>>,
    text_embedding_model: Arc<Mutex<TextEmbedding>>,
}

impl SearchRepositoryImpl {
    pub fn new(
        search_vector_db_service: Arc<dyn SearchVectorDbService>,
        markdown_splitter: Arc<MarkdownSplitter<Tokenizer>>,
        text_embedding_model: Arc<Mutex<TextEmbedding>>,
    ) -> Self {
        Self {
            search_vector_db_service,
            markdown_splitter,
            text_embedding_model,
        }
    }
}

#[async_trait]
impl SearchRepository for SearchRepositoryImpl {
    async fn index_post(&self, post: Post) -> Result<(), SearchError> {
        let markdown_content = format!(
            "# {} \n\n{}\n\n---\n\n{}",
            post.info.title, post.info.description, post.content
        );
        let chunks = self.split_markdown(&markdown_content);
        let embeddings = self.embed_chunks(&chunks).await?;

        self.search_vector_db_service
            .delete_vectors_by_post_id(post.id)
            .await?;
        self.search_vector_db_service
            .insert_vectors(post.id, &embeddings)
            .await
    }

    async fn search_posts(
        &self,
        keyword: &str,
        scope: &Option<Vec<i32>>,
    ) -> Result<Vec<i32>, SearchError> {
        let query_string = format!("{}{}", QUERY_INSTRUCTION, keyword);
        let embeddings = self.embed_chunks(&vec![query_string]).await?;
        let query_vector = embeddings
            .into_iter()
            .next()
            .ok_or(SearchError::Unexpected(anyhow!(
                "Failed to generate embedding for query"
            )))?;

        self.search_vector_db_service
            .search_similar_posts(&query_vector, scope)
            .await
    }
}

impl SearchRepositoryImpl {
    fn split_markdown(&self, markdown: &str) -> Vec<String> {
        self.markdown_splitter
            .chunks(&markdown)
            .map(|s| s.to_string())
            .collect()
    }

    async fn embed_chunks(&self, chunks: &Vec<String>) -> Result<Vec<Vec<f32>>, SearchError> {
        let model = self.text_embedding_model.clone();
        let chunks = chunks.clone();

        spawn_blocking(move || {
            let mut model = model.blocking_lock();
            model.embed(chunks, None)
        })
        .await
        .map_err(|e| SearchError::Unexpected(e.into()))?
        .map_err(|e| SearchError::Unexpected(e.into()))
    }
}
