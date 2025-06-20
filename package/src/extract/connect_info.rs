use axum::extract::ConnectInfo as _ConnectInfo;
use axum_core::extract::FromRequestParts;
use http::request::Parts;

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Extractor for getting connection information produced
/// by a [`Connected`](axum::extract::connect_info::Connected).
///
/// Check [`ConnectInfo`](axum::extract::ConnectInfo) for more information.
///
/// ## Example
///
/// ```no_run
/// use std::net::{IpAddr, SocketAddr};
///
/// use axum::{
///     Router,
///     routing::get,
///     extract::connect_info::IntoMakeServiceWithConnectInfo,
/// };
/// use jder_axum::extract::ConnectInfo;
///
/// async fn route(
///     ConnectInfo(addr): ConnectInfo<SocketAddr>
/// ) {
///     let ip: IpAddr = addr.ip();
///     let is_ipv4: bool = addr.is_ipv4();
///     let is_ipv6: bool = addr.is_ipv6();
///     let port: u16 = addr.port();
/// }
///
/// async fn example(){
///     let router: IntoMakeServiceWithConnectInfo<Router, SocketAddr> =
///         Router::new()
///             .route("/", get(route))
///             .into_make_service_with_connect_info::<SocketAddr>();
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ConnectInfo<T>(pub T);

impl<S, T> FromRequestParts<S> for ConnectInfo<T>
where
    S: Send + Sync,
    T: Clone + Send + Sync + 'static,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _ConnectInfo::<T>::from_request_parts(parts, state).await {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

axum_core::__impl_deref!(ConnectInfo);
