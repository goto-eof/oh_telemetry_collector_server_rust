use super::controller_common;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};

pub async fn store_data(esempio: Esempio) -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(generate_string(esempio.data)));
}

fn generate_string(esempio: Vec<KeyValue>) -> String {
    let mut result = "".to_string();
    for tuple in esempio {
        result = format!("key={}, value={}; {}", tuple.key, tuple.value, result);
    }
    return result.to_owned();
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Esempio {
    data: Vec<KeyValue>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KeyValue {
    code: String,
    key: String,
    value: String,
}
