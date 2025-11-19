#![allow(dead_code)]

use thiserror::Error;
use std::convert::Infallible;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An error occurred")]
    GenericError,
}

impl From<Infallible> for AppError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}