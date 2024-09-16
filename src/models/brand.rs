use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Brand {
    pub brand_image: String,
    pub brand_name: String,
    pub proof: String,
    pub source: String,
}
