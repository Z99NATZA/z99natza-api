use bytes::Bytes;
use hyper::{Request, body::Incoming};
use http_body_util::BodyExt;

use crate::app::AppResult;

pub struct FullBody;

impl FullBody {
    pub async fn new(mut req: Request<Incoming>) -> AppResult<Bytes> {
        let result = req.body_mut().collect().await?.to_bytes();
        Ok(result)
    }
}

