use serde_json::Value;
use serde_json;
use hyper;
use hyper::{StatusCode, Uri, Request, Method as RequestMethod, Response};
use hyper::header::*;
use hyper_rustls::HttpsConnector;
use tokio_core::reactor;
use futures;
use futures::Future;
use futures::Stream;
use serde_urlencoded;

///
/// Contains the base URL to access the API
///
pub const API_HOST: &'static str = "https://api.wit.ai";

///
/// Contains the version of the API to use. This is required.
///
pub const API_VERSION: &'static str = "20160516";

///
/// Describes HTTP access methods
///
pub enum Method {
    Get,
    Post
}

///
/// Represents an error in somunicating with the server
///
#[derive(Debug)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
    pub code: u16
}

///
/// Handles comunication with the wit.ai servers.
/// The result is a decoded JSON object.
///
pub fn request(
    token: String, method: Method, path: String,
    params: Option<Value>, payload: Option<Value>

) -> Result<Value, HttpError>
{
    let url = &*build_url(path, params);

    let mut core = reactor::Core::new().unwrap();
    let url: Uri = (url).parse().unwrap();

    let config =
        hyper::Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()));
    let client = config.build(&core.handle());

    let body = build_body(payload.unwrap_or(json!({})));

    let request_method = match method {
        Method::Get => RequestMethod::Get,
        Method::Post => RequestMethod::Post
    };

    let mut request = Request::new(request_method, url);
    request.set_body(body);

    let authorization_header = format!("Bearer {}", &token[..]);
    request.headers_mut().set(Authorization(authorization_header));

    let accept_header =
        format!("application/vnd.wit.{}+json", API_VERSION);
    let accept_header =
        Header::parse_header(&Raw::from(accept_header.as_bytes()))
        .unwrap_or(Accept(vec![]));
    request.headers_mut().set(accept_header);
    request.headers_mut().set(
        ContentType(
            "application/json".parse::<hyper::mime::Mime>().unwrap()
        )
    );

    let request = client.request(request);

    let response = match core.run(request) {
        Ok(response) => response,
        Err(error) => {
            let reason = format!(
                "Could not connect to server - {}",
                error
            );

            return Err(
                HttpError {
                    message: reason,
                    status: 0,
                    code: 100
                }
            )
        }
    };

    if response.status() != StatusCode::Ok {
        let reason = format!(
            "Server responded with error: {}",
            response.status()
        );


        return Err(
            HttpError {
                message: reason,
                status: response.status().as_u16(),
                code: 101
            }
        );
    };

    Ok(deserialize_response(response))
}

///
/// Given a path and params it builds a fully qualified URL
///
fn build_url(path: String, params: Option<Value>) -> String {
    let mut url = API_HOST.to_string();

    url.push_str(&path);

    let params = match params {
        Some(json) => json,
        _ => json!({})
    };
    let url_encoded_params = match serde_urlencoded::to_string(&params) {
        Ok(encoded_params) => encoded_params,
        Err(_error) => "".to_string()
    };

    if url_encoded_params.len() > 0 {
        url.push_str("?");
    }
    url.push_str(&url_encoded_params);

    url
}


///
/// Converts the input map into a JSON serialized string of the input map
///
fn build_body(payload: Value) -> String {
    payload.to_string()
}

///
/// Converts the raw response string to a map
///
fn deserialize_response(response: Response) -> Value {
    if response.status() != StatusCode::Ok {
        return json!({});
    }

    let mut body = String::new();
    let response =
        response
        .body()
        .fold(Vec::new(), |mut acc, chunk| {
            acc.extend_from_slice(&*chunk);
            futures::future::ok::<_, hyper::Error>(acc)
        })
        .map(|body_bytes| {
            let string = match String::from_utf8(body_bytes) {
                Ok(body) => body,
                Err(_) => "".to_string()
            };
            body.push_str(&string[..]);
        })
        .poll();

    match response {
        Ok(_) => (),
        Err(_) => return json!({})
    }

    match serde_json::from_str(&body[..]) {
        Ok(body) => body,
        Err(_error) => json!({})
    }
}
