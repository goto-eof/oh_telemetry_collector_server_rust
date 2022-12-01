use warp::{Filter, Rejection, Reply};

use crate::controller::controller_common::generate_response;

use super::routes_telemetry::get_telemetry_routes;

pub async fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors_allowed_origins: Vec<&str> = vec![];

    let cors_allowed_headers: Vec<&str> = vec![];

    let cors_allowed_methods: Vec<&str> = vec!["GET", "POST", "PUT", "DELETE"];

    let any_origin_3 = warp::cors()
        .allow_origins(cors_allowed_origins)
        .allow_headers(cors_allowed_headers)
        .allow_methods(cors_allowed_methods)
        .allow_credentials(true);

    get_telemetry_routes()
        .with(&any_origin_3)
        .with(warp::log("api"))
        .recover(handle_rejection)
}

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        return generate_response(Ok("Not found".to_string()));
    } else {
        return generate_response(Ok("Page not found?".to_string()));
    }
}
