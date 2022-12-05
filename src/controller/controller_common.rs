use serde::Serialize;
use warp::reply::json;
use warp::{Rejection, Reply};

use super::response::telemetry_generic_response::TelemetryGenericResponse;

pub fn generate_response<T: Serialize>(
    data_wrapped: Result<T, Rejection>,
) -> Result<impl Reply, Rejection> {
    let response = match data_wrapped {
        Ok(result) => json::<_>(&TelemetryGenericResponse {
            status: true,
            result: &result,
        }),
        Err(err) => return Err(err),
    };
    Ok(response)
}
