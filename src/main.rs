use crate::route::routes_util::init_routes;
mod controller;
mod route;

#[tokio::main]
async fn main() {
    println!("================================");
    println!("server started on port [8013] :)");
    println!("================================");

    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], 8013))
        .await;
}
