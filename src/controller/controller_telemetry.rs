use std::collections::HashMap;

use super::controller_common;
use crate::dao::service_telemetry::{store_data_db, telemetry_retrieve_num_comp_ram};
use log::debug;
use warp::{Rejection, Reply};

pub async fn store_data(
    telemetry_insert_request: HashMap<String, HashMap<String, Option<String>>>,
) -> Result<impl Reply, Rejection> {
    debug!(
        "received request for store_data(): {:?}",
        &telemetry_insert_request
    );
    return controller_common::generate_response(Ok(store_data_db(telemetry_insert_request).await));
}

pub async fn retrieve_num_comp_ram() -> Result<impl Reply, Rejection> {
    debug!("retrieve_num_comp_ram()");
    let result = telemetry_retrieve_num_comp_ram().await;
    return controller_common::generate_response(Ok(result.unwrap()));
}
