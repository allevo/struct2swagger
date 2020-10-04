# struct2swagger [![Build Status](https://travis-ci.org/allevo/struct2swagger.svg?branch=master)](https://travis-ci.org/allevo/struct2swagger)

Utilities for generating OpenAPI Specification from your structures

## Install
```
cargo add struct2swagger_derive struct2swagger
```

## Usage

```rust

#[macro_use]
extern crate struct2swagger_derive;
#[macro_use]
extern crate struct2swagger;
#[macro_use]
extern crate serde_json;

use struct2swagger::{JsonSchemaDefinition, QueryDefinition, swagger_object::SwaggerObject};
use serde_json::Result;

#[derive(Deserialize, Swagger)]
pub struct Who {
    pub name: Option<String>,
}
#[derive(Serialize, Swagger)]
pub struct HelloWorldResponse {
    pub say: String,
}

fn get_openapi_spec() -> String {
  let mut swagger_object = SwaggerObject::new(
    "the webserver name", // title
    "1.0.0" // version
  );

  swagger_add_router!(
      swagger_object, // obj
      "GET", // method
      "/", // path
      Who, // query parameters
      200, // expected status code
      "say", //  description
      HelloWorldResponse // struct in output
  );

  let json = serde_json::to_string(&address).unwrap();

  json
}
```

For other examples see [tests](./struct2swagger_derive/tests/swagger.rs)

## Contributing

Every contribution is welcomed: Open an issue and fire a PR!

## License

MIT as described [here](./LICENSE)
