use super::controller_common;
use crate::dao::service_telemetry::store_data_db;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};

pub async fn store_data(esempio: Esempio) -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(store_data_db(esempio.data).await));
}

fn generate_string(esempio: Vec<KeyValue>) -> String {
    let mut result = "".to_string();
    for tuple in esempio {
        result = format!(
            "key={}, value={}; {}",
            tuple.key,
            tuple.value.unwrap_or("".to_string()),
            result
        );
    }
    return result.to_owned();
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Esempio {
    data: Vec<KeyValue>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KeyValue {
    pub code: String,
    pub key: String,
    pub value: Option<String>,
}
