use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    URLError(#[from] url::ParseError),

    #[error(transparent)]
    WSError(#[from] tokio_tungstenite::tungstenite::Error),
}
