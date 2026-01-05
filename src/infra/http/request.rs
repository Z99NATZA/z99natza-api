use hyper::{Request, body::Incoming};
use serde::de::DeserializeOwned;
use crate::app::AppError;
use http_body_util::BodyExt;

pub async fn json<T: DeserializeOwned>(req: Request<Incoming>) -> Result<T, AppError> {
    let body = req.into_body().collect().await?.to_bytes();
    let obj = serde_json::from_slice::<T>(&body)?;
    Ok(obj)
}