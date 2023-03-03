use axum::{extract::{FromRequest, RequestParts}, async_trait, BoxError, Json};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T,B> FromRequest<B> for ValidatedJson<T>
where
	T: DeserializeOwned + Validate,
	B: http_body::Body + Send,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	type Rejection = (StatusCode, String);

	async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
		let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
			let message = format!("Json parse error: [{}]", rejection);
			(StatusCode::BAD_REQUEST, message)
		})?;
		value.validate().map_err(|rejection| {
			let message = format!("Validation error: [{}]", rejection);
			(StatusCode::BAD_REQUEST, message)
		})?;
		Ok(ValidatedJson(value))
	}
}