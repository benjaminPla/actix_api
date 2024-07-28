use serde::Serialize;

#[derive(Serialize)]
pub enum StatusOption {
    UP,
}

#[derive(Serialize)]
pub struct Status {
    pub status: StatusOption,
}
