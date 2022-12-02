use super::{controller_common, request::telemetry_insert_request::TelemetryInsertrequest};
use crate::dao::service_telemetry::store_data_db;
use log::debug;
use warp::{Rejection, Reply};

pub async fn store_data(
    telemetry_insert_request: TelemetryInsertrequest,
) -> Result<impl Reply, Rejection> {
    debug!(
        "received request for store_data(): {:?}",
        &telemetry_insert_request
    );
    return controller_common::generate_response(Ok(
        store_data_db(telemetry_insert_request.data).await
    ));
}
