pub mod clan_info;
pub mod login;
pub mod search;
pub mod window;

#[derive(serde::Serialize)]
pub struct Response {
    pub status: i32,
    pub message: String,
}
