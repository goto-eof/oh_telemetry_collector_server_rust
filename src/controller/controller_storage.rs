use super::controller_common;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};

pub async fn store_data(name: String, esempio: Esempio) -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(format!("{} {}", name, esempio.last_name)));
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Esempio {
    last_name: String,
}
