pub struct ImageMetaData {
    pub id: i32,
    pub mime_type: String,
    /// Size of the stored data in bytes, or `None` when the data cannot be read (for example
    /// when the file is missing). The record itself is still meaningful without it, so an
    /// unreadable file does not fail the request that reads this metadata.
    pub size: Option<i64>,
}
