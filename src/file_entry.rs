#[derive(serde::Deserialize, Debug)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub size: i64
}