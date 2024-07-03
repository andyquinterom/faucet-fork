use std::convert::Infallible;

use crate::client::ExclusiveBody;

pub enum BadRequestReason {
    MissingHeader(&'static str),
    InvalidHeader(&'static str),
}

pub type FaucetResult<T> = std::result::Result<T, FaucetError>;

pub enum FaucetError {
    PoolBuild(deadpool::managed::BuildError),
    PoolTimeout(deadpool::managed::TimeoutType),
    PoolPostCreateHook,
    PoolClosed,
    PoolNoRuntimeSpecified,
    Io(std::io::Error),
    Unknown(String),
    HostParseError(std::net::AddrParseError),
    Hyper(hyper::Error),
    BadRequest(BadRequestReason),
    InvalidHeaderValues(hyper::header::InvalidHeaderValue),
    Http(hyper::http::Error),
    MissingArgument(&'static str),
    DuplicateRoute(&'static str),
}

impl From<hyper::header::InvalidHeaderValue> for FaucetError {
    fn from(e: hyper::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValues(e)
    }
}

impl From<hyper::http::Error> for FaucetError {
    fn from(e: hyper::http::Error) -> Self {
        Self::Http(e)
    }
}

impl From<deadpool::managed::PoolError<FaucetError>> for FaucetError {
    fn from(value: deadpool::managed::PoolError<FaucetError>) -> Self {
        match value {
            deadpool::managed::PoolError::Backend(e) => e,
            deadpool::managed::PoolError::Timeout(e) => Self::PoolTimeout(e),
            deadpool::managed::PoolError::Closed => Self::PoolClosed,
            deadpool::managed::PoolError::PostCreateHook(_) => Self::PoolPostCreateHook,
            deadpool::managed::PoolError::NoRuntimeSpecified => Self::PoolNoRuntimeSpecified,
        }
    }
}

impl From<Infallible> for FaucetError {
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible error")
    }
}

impl From<deadpool::managed::BuildError> for FaucetError {
    fn from(e: deadpool::managed::BuildError) -> Self {
        Self::PoolBuild(e)
    }
}

impl From<std::io::Error> for FaucetError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::net::AddrParseError> for FaucetError {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::HostParseError(e)
    }
}

impl From<hyper::Error> for FaucetError {
    fn from(e: hyper::Error) -> Self {
        Self::Hyper(e)
    }
}

impl std::fmt::Display for FaucetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PoolBuild(e) => write!(f, "Pool build error: {}", e),
            Self::PoolTimeout(e) => write!(f, "Pool timeout error: {:?}", e),
            Self::PoolPostCreateHook => write!(f, "Pool post create hook error"),
            Self::PoolClosed => write!(f, "Pool closed error"),
            Self::PoolNoRuntimeSpecified => write!(f, "Pool no runtime specified error"),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Unknown(e) => write!(f, "Unknown error: {}", e),
            Self::HostParseError(e) => write!(f, "Error parsing host address: {}", e),
            Self::Hyper(e) => write!(f, "Hyper error: {}", e),
            Self::Http(e) => write!(f, "Http error: {}", e),
            Self::InvalidHeaderValues(e) => write!(f, "Invalid header values: {}", e),
            Self::MissingArgument(s) => write!(f, "Missing argument: {}", s),
            Self::DuplicateRoute(route) => writeln!(f, "Route '{route}' is duplicated"),
            Self::BadRequest(r) => match r {
                BadRequestReason::MissingHeader(header) => {
                    write!(f, "Missing header: {}", header)
                }
                BadRequestReason::InvalidHeader(header) => {
                    write!(f, "Invalid header: {}", header)
                }
            },
        }
    }
}

impl std::fmt::Debug for FaucetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MissingArgument(s) => write!(f, "Missing argument: {}", s),
            Self::PoolTimeout(e) => write!(f, "Pool timeout error: {:?}", e),
            Self::PoolPostCreateHook => write!(f, "Pool post create hook error"),
            Self::PoolClosed => write!(f, "Pool closed error"),
            Self::PoolNoRuntimeSpecified => write!(f, "Pool no runtime specified error"),
            Self::PoolBuild(e) => write!(f, "Pool build error: {:?}", e),
            Self::Io(e) => write!(f, "IO error: {:?}", e),
            Self::Unknown(e) => write!(f, "Unknown error: {:?}", e),
            Self::HostParseError(e) => write!(f, "Error parsing host address: {:?}", e),
            Self::Hyper(e) => write!(f, "Hyper error: {:?}", e),
            Self::Http(e) => write!(f, "Http error: {:?}", e),
            Self::InvalidHeaderValues(e) => write!(f, "Invalid header values: {:?}", e),
            Self::DuplicateRoute(route) => writeln!(f, "Route '{route}' is duplicated"),
            Self::BadRequest(r) => match r {
                BadRequestReason::MissingHeader(header) => {
                    write!(f, "Missing header: {}", header)
                }
                BadRequestReason::InvalidHeader(header) => {
                    write!(f, "Invalid header: {}", header)
                }
            },
        }
    }
}

impl std::error::Error for FaucetError {}

impl FaucetError {
    pub fn no_sec_web_socket_key() -> Self {
        Self::BadRequest(BadRequestReason::MissingHeader("Sec-WebSocket-Key"))
    }
    pub fn unknown(s: impl ToString) -> Self {
        Self::Unknown(s.to_string())
    }
}

impl From<FaucetError> for hyper::Response<ExclusiveBody> {
    fn from(val: FaucetError) -> Self {
        let mut resp = hyper::Response::new(ExclusiveBody::plain_text(val.to_string()));
        *resp.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faucet_error() {
        let err = FaucetError::unknown("test");
        assert_eq!(err.to_string(), "Unknown error: test");
    }

    #[test]
    fn test_faucet_error_debug() {
        let err = FaucetError::unknown("test");
        assert_eq!(format!("{:?}", err), r#"Unknown error: "test""#);
    }

    #[test]
    fn test_faucet_error_from_hyper_error() {
        let err = hyper::Request::builder()
            .uri("INVALID URI")
            .body(())
            .unwrap_err();

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_io_error() {
        let err = std::io::Error::new(std::io::ErrorKind::Other, "test");

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_pool_error() {
        let err = deadpool::managed::PoolError::Backend(FaucetError::unknown("test"));

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_pool_build_error() {
        let err = deadpool::managed::BuildError::NoRuntimeSpecified;

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_pool_timeout_error() {
        let err = deadpool::managed::PoolError::<FaucetError>::Timeout(
            deadpool::managed::TimeoutType::Create,
        );

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_pool_closed_error() {
        let err = deadpool::managed::PoolError::<FaucetError>::Closed;

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_pool_post_create_hook_error() {
        let err = deadpool::managed::PoolError::<FaucetError>::PostCreateHook(
            deadpool::managed::HookError::StaticMessage("test"),
        );

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_pool_no_runtime_specified_error() {
        let err = deadpool::managed::PoolError::<FaucetError>::NoRuntimeSpecified;

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_hyper_invalid_header_value_error() {
        let err = hyper::header::HeaderValue::from_bytes([0x00].as_ref()).unwrap_err();

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_from_addr_parse_error() {
        let err = "INVALID".parse::<std::net::SocketAddr>().unwrap_err();

        let err: FaucetError = From::from(err);
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_displat_missing_header() {
        let err = FaucetError::BadRequest(BadRequestReason::MissingHeader("test"));
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_faucet_error_displat_invalid_header() {
        let err = FaucetError::BadRequest(BadRequestReason::InvalidHeader("test"));
        format!("{:?}", err);
        format!("{}", err);
    }

    #[test]
    fn test_from_fauct_error_to_hyper_response() {
        let err = FaucetError::unknown("test");
        let resp: hyper::Response<ExclusiveBody> = err.into();
        assert_eq!(resp.status(), hyper::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
