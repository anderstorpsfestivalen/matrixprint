use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    URLError(#[from] url::ParseError),

    #[error(transparent)]
    WSError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    OpenPrinterError(#[from] std::io::Error),

    #[error(transparent)]
    #[cfg(feature = "rpi")]
    GPIOError(#[from] rppal::gpio::Error),
}
