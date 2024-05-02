use axum::http::StatusCode;

#[derive(serde::Serialize)]
pub struct Response<T> {
    pub status: i32,
    pub code: i32,
    pub error: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn new_ok(data: T) -> Self {
        Self {
            status: 200,
            code: 0,
            error: None,
            data: Some(data),
        }
    }
    pub fn new_error(code: i32, status: StatusCode, error: String) -> Self {
        Self {
            status: status.as_u16() as i32,
            code,
            error: Some(error),
            data: None,
        }
    }
}
