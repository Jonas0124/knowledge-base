use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct EmailRes {
    pub RequestId: Option<String>,
    pub EnvId: Option<String>,
}