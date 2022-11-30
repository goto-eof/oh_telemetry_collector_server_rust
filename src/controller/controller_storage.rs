use super::controller_common;
use warp::{Rejection, Reply};

pub async fn store_data(name: String) -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(name));
}
