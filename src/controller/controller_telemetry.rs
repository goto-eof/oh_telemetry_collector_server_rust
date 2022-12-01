use super::{controller_common, request::telemetry_insert_request::TelemetryInsertrequest};
use crate::dao::service_telemetry::store_data_db;
use warp::{Rejection, Reply};

pub async fn store_data(esempio: TelemetryInsertrequest) -> Result<impl Reply, Rejection> {
    return controller_common::generate_response(Ok(store_data_db(esempio.data).await));
}
