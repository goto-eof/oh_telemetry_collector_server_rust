use serde::Serialize;
use warp::reply::json;
use warp::{Rejection, Reply};

pub fn generate_response<T: Serialize>(
    data_wrapped: Result<T, Rejection>,
) -> Result<impl Reply, Rejection> {
    let response = match data_wrapped {
        Ok(result) => json::<_>(&Response {
            success: true,
            result: &result,
        }),
        Err(err) => return Err(err),
    };
    Ok(response)
}
#[derive(Serialize, Debug)]
struct Response<T> {
    success: bool,
    result: T,
}
