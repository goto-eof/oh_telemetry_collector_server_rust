use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TelemetryInsertrequest {
    pub data: Vec<KeyValue>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KeyValue {
    pub code: String,
    pub key: String,
    pub value: Option<String>,
}
