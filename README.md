# rocket_empty

`rocket_empty` provides a singular struct `EmptyResponse` which has a Responder implementation that returns "204 No Content". In addition to this, it also implements a schema for okapi.

I got tired of copying the same struct and implementation between projects, so here it is.

## Usage

```rust
use rocket_empty::EmptyResponse;

#[openapi(tag = "Tagged")]
#[get("/test")]
pub async fn test() -> EmptyResponse {
    EmptyResponse
    // Responds with "204 No Content"
}
```
