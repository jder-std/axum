# JDER axum

A response builder for axum.

This package includes different axum response builders, extractors and layers based on the JSON response structure specified in [JSON Data Error Response (JDER)](https://github.com/jder-std/spec). With the builders and extractors provided, various kinds of responses can be created easily instead of sending plain text responses.

## Quick Start

Create a JSON response for an axum route:

```rust
use jder_axum::response::{
    Response,
    json::CreateJsonResponse,
};

async fn route() -> Response {
    CreateJsonResponse::dataless().send()
}
```

And the response will be shown as below:

```json
{
    "success": true,
    "data": null,
    "error": null
}
```

## License

This project is licensed under the terms of the MIT license.
