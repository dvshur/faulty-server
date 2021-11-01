use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Response {
    ok: bool,
}

pub fn ok() -> Response {
    Response { ok: true }
}
