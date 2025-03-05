use fastly::http::{header, HeaderName, StatusCode};
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    Ok(
        if req.get_header(HeaderName::from_static("~^.^~")).is_some() {
            Response::from_status(StatusCode::OK).with_body_text_html("I like your cat\n")
        } else {
            Response::from_status(StatusCode::TEMPORARY_REDIRECT)
                .with_header(header::LOCATION, "https://http.cat")
        },
    )
}
