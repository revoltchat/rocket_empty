use okapi::openapi3::RefOr;
use rocket::{
    response::{self, Responder},
    Request, Response,
};
use rocket_okapi::okapi::openapi3;

pub struct EmptyResponse;
pub type NoContent = EmptyResponse;

impl<'r> Responder<'r, 'static> for EmptyResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(rocket::http::Status { code: 204 })
            .ok()
    }
}

#[cfg(feature = "schema")]
impl rocket_okapi::response::OpenApiResponderInner for EmptyResponse {
    fn responses(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> std::result::Result<openapi3::Responses, rocket_okapi::OpenApiError> {
        let mut responses = okapi::Map::new();

        responses.insert(
            "204".to_string(),
            RefOr::Object(openapi3::Response {
                description: "Success".to_string(),
                ..Default::default()
            }),
        );

        Ok(openapi3::Responses {
            responses,
            ..Default::default()
        })
    }
}
