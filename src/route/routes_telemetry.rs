use crate::controller::controller_telemetry;
use warp::{Filter, Rejection, Reply};

pub fn get_telemetry_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let routes = warp::path("collect")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(controller_telemetry::store_data);

    return routes;
}
