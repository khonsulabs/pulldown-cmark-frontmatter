use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExampleAttributes {
    pub author: String,
    pub last_updated_at: String,
}
