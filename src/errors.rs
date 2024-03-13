use thiserror::Error;
use reqwest::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error(r#"Cannot get response from API:
    {0}"#)]
    RequestError(Error)
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(r#"Unsupported currency code: {0}"#)]
    UnsupportedCode(String),
    #[error(r#"Malformed request to API"#)]
    MalformedRequest(),
    #[error(r#"Invalid API key"#)]
    InvalidKey(),
    #[error(r#"Inactive account - email address wasn't confirmed"#)]
    InactiveAccount(),
    #[error(r#"Quota reached: Your account has reached the maximum number of requests allowed by your plan"#)]
    QuotaReached(),
    #[error(r#"Unknown API error - {0}"#)]
    UnknownError(String)
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error(r#"Cannot find conversion data for {0} -> {1}"#)]
    CurrencyNotFound(String, String)
}

#[derive(Error, Debug)]
pub enum InputError {
    #[error(r#"Currency code has to be in all uppercase letters, for example - USD, EUR"#)]
    InvalidCode(),
    #[error(r#"Amount must be a valid positive number"#)]
    InvalidAmount(),
    #[error(r#"There was an error when reading input - {0}"#)]
    ReadLineError(String)
}

#[derive(Error, Debug)]
pub enum ApiKeyError {
    #[error(r#"Cannot find the API key. You can provide it by creating EXCHANGERATE_API_KEY envrionment variable containing the key."#)]
    KeyNotFound()
}