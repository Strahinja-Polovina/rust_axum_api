use serde::Serialize;

#[derive(Serialize)]
pub struct Message<T> {
    pub status: u16,
    pub message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
