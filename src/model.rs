use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct District {
    num: String,
    name: String,
    candidate: String,
    streets: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelController {
    district_store: Vec<District>,
}

// Constructor
impl ModelController {
    pub fn new() -> Self {
        Self {
            district_store: vec![],
        }
    }
}
