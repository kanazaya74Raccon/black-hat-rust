use crate::api;
use std::{sync::Arc, time::Duration};
use warp::http::StatusCode;

pub async fn jobs(state: Arc<crate::AppState>) -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    for _ in 0..5u64 {
        match state.service.get_job().await? {
            Some(job) => {
                let res = api::Response::ok(job);
                let res_json = warp::reply::json(&res);
                return Ok(warp::reply::with_status(res_json, StatusCode::OK));
            }
            None => tokio::time::sleep(sleep_for).await,
        }
    }

    let res = api::Response::<Option<()>>::ok(None);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
