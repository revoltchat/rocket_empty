use rocket::{
    response::{self, Responder},
    Request, Response,
};

pub struct EmptyResponse;

impl<'r> Responder<'r, 'static> for EmptyResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(rocket::http::Status { code: 204 })
            .ok()
    }
}

#[cfg(feature = "schema")]
use revolt_okapi::openapi3::RefOr;

#[cfg(feature = "schema")]
use revolt_rocket_okapi::revolt_okapi::openapi3;

#[cfg(feature = "schema")]
impl revolt_rocket_okapi::response::OpenApiResponderInner for EmptyResponse {
    fn responses(
        _gen: &mut revolt_rocket_okapi::gen::OpenApiGenerator,
    ) -> std::result::Result<openapi3::Responses, revolt_rocket_okapi::OpenApiError> {
        let mut responses = revolt_okapi::Map::new();

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
