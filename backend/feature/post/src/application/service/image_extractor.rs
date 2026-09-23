use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

/// Matches the trailing path segments of an image URL and captures its id, for example the `42`
/// in `/image/42`.
static IMAGE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\/[^\/\s]+)*\/image\/(\d+)"#).unwrap());

/// Matches a markdown image whose target is an [`IMAGE_REGEX`] match.
static MARKDOWN_IMAGE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "{}{}{}",
        r#"!\[.*\]\("#,
        IMAGE_REGEX.as_str(),
        r#"\)"#
    ))
    .unwrap()
});

pub struct ImageExtractor;

impl ImageExtractor {
    pub fn extract_preview_image_id(preview_image_url: &Option<String>) -> Option<i32> {
        let url = preview_image_url.as_ref()?;
        let cap = IMAGE_REGEX.captures(url)?;
        cap.get(2)?.as_str().parse::<i32>().ok()
    }

    pub fn extract_image_ids(content: &str, preview_image_url: &Option<String>) -> Vec<i32> {
        let mut image_ids = HashSet::new();

        for cap in MARKDOWN_IMAGE_REGEX.captures_iter(content) {
            if let Some(id_str) = cap.get(2)
                && let Ok(id) = id_str.as_str().parse::<i32>()
            {
                image_ids.insert(id);
            }
        }

        if let Some(id) = Self::extract_preview_image_id(preview_image_url) {
            image_ids.insert(id);
        }

        image_ids.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ImageExtractor;

    #[test]
    fn test_extract_md_image_ids() {
        let content = r#"
            ![alt text](/image/123)
            ![another image](/path/to/image/456)
            ![invalid image](/path/to/image/notanumber)
            ![no id image](/path/to/image/)
            ![not an image](http://example.com/image/789)
            ![not an image](http://example.com/notanimage/123)
            ![not an image](//path/image/notanumber)
        "#;
        let mut ids = ImageExtractor::extract_image_ids(content, &None);
        ids.sort();
        assert_eq!(ids, vec![123, 456]);
    }

    #[test]
    fn test_extract_preview_image_id() {
        let content = "Some content without images";
        let preview_image_url = Some("/path/to/image/789".to_string());
        let ids = ImageExtractor::extract_image_ids(content, &preview_image_url);
        assert_eq!(ids, vec![789]);
    }

    #[test]
    fn test_extract_preview_image_id_only() {
        assert_eq!(
            ImageExtractor::extract_preview_image_id(&Some("/image/42".to_string())),
            Some(42)
        );
        assert_eq!(
            ImageExtractor::extract_preview_image_id(&Some(
                "https://example.com/image/7".to_string()
            )),
            Some(7)
        );
        assert_eq!(
            ImageExtractor::extract_preview_image_id(&Some(
                "https://example.com/photo.png".to_string()
            )),
            None
        );
        assert_eq!(ImageExtractor::extract_preview_image_id(&None), None);
    }
}
