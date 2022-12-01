use crate::controller::controller_storage;
use warp::{Filter, Rejection, Reply};

pub fn get_telemetry_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let routes = warp::path("telemetry")
        .and(warp::post())
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(controller_storage::store_data);

    return routes;
}
