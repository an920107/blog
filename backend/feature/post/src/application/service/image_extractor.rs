use regex::Regex;
use std::collections::HashSet;

pub struct ImageExtractor;

impl ImageExtractor {
    pub fn extract_image_ids(content: &str, preview_image_url: &Option<String>) -> Vec<i32> {
        let mut image_ids = HashSet::new();

        let image_re = Regex::new(r#"(\/[^\/\s]+)*\/image\/(\d+)"#).unwrap();
        let md_image_re = Regex::new(&format!(
            "{}{}{}",
            r#"!\[.*\]\("#,
            image_re.as_str(),
            r#"\)"#
        ))
        .unwrap();

        for cap in md_image_re.captures_iter(content) {
            if let Some(id_str) = cap.get(2) {
                if let Ok(id) = id_str.as_str().parse::<i32>() {
                    image_ids.insert(id);
                }
            }
        }

        if let Some(url) = preview_image_url {
            if let Some(cap) = image_re.captures(url) {
                if let Some(id_str) = cap.get(2) {
                    if let Ok(id) = id_str.as_str().parse::<i32>() {
                        image_ids.insert(id);
                    }
                }
            }
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
}
