use std::path::PathBuf;

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use text_splitter::{ChunkConfig, MarkdownSplitter};
use tokenizers::Tokenizer;

#[derive(Clone)]
pub struct EmbeddingConfiguration {
    model: EmbeddingModel,
    hf_model_name: String,
    cache_dir: String,
    max_token: usize,
    overlap_token: usize,
}

impl EmbeddingConfiguration {
    pub fn new() -> Self {
        let cache_dir = std::env::var("EMBEDDING_CACHE_DIR")
            .unwrap_or_else(|_| "./embedding_cache".to_string());

        Self {
            model: EmbeddingModel::BGELargeZHV15,
            hf_model_name: "BAAI/bge-large-zh-v1.5".to_string(),
            cache_dir,
            max_token: 450,
            overlap_token: 50,
        }
    }

    pub fn create_embedding_options(&self) -> TextEmbedding {
        let options = InitOptions::new(self.model.clone())
            .with_show_download_progress(true)
            .with_cache_dir(PathBuf::from(self.cache_dir.clone()));

        TextEmbedding::try_new(options.clone()).expect("Failed to initialize TextEmbedding")
    }

    pub fn create_markdown_splitter(&self) -> MarkdownSplitter<Tokenizer> {
        let tokenizer = Tokenizer::from_pretrained(self.hf_model_name.clone(), None)
            .expect("Failed to load tokenizer");

        let chunk_config = ChunkConfig::new(self.max_token)
            .with_sizer(tokenizer)
            .with_overlap(self.overlap_token)
            .expect("Failed to set overlap for chunk config");

        MarkdownSplitter::new(chunk_config)
    }
}
